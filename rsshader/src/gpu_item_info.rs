use std::mem::MaybeUninit;

const ITEM_CAPACITY: usize = 256;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GPUItemInfo {
    pub id: u128,
    pub dependencies: &'static [&'static GPUItemInfo],
    pub wgsl_declaration: &'static str,
}
impl GPUItemInfo {
    #[allow(invalid_value)]
    pub const fn resolve_dependencies(
        items: &[&'static GPUItemInfo],
    ) -> (MaybeUninit<[&'static GPUItemInfo; ITEM_CAPACITY]>, usize) {
        let mut inverted_output = (
            unsafe { MaybeUninit::<[&'static GPUItemInfo; ITEM_CAPACITY]>::uninit().assume_init() },
            0,
        );

        let mut pushed_item_ids = [0; ITEM_CAPACITY];
        let mut pushed_item_ids_len = 1;

        let mut items_to_push =
            unsafe { MaybeUninit::<[&'static GPUItemInfo; ITEM_CAPACITY]>::uninit().assume_init() };
        let mut items_to_push_len = 0;

        {
            let mut i = 0;
            while i < items.len() {
                items_to_push[items_to_push_len] = items[i];
                items_to_push_len += 1;

                i += 1;
            }
        }
        {
            while items_to_push_len > 0 {
                items_to_push_len -= 1;
                let item = items_to_push[items_to_push_len];

                if 'not_in_pushed_items: {
                    let mut i = 0;
                    while i < pushed_item_ids_len {
                        if pushed_item_ids[i] == item.id {
                            break 'not_in_pushed_items false;
                        }

                        i += 1;
                    }

                    true
                } {
                    pushed_item_ids[pushed_item_ids_len] = item.id;
                    pushed_item_ids_len += 1;

                    {
                        let mut i = 0;
                        while i < item.dependencies.len() {
                            items_to_push[items_to_push_len] = item.dependencies[i];
                            items_to_push_len += 1;

                            i += 1;
                        }
                    }

                    inverted_output.0[inverted_output.1] = item;
                    inverted_output.1 += 1;
                }
            }
        }

        let output = {
            let mut i = 0;
            while i < inverted_output.1 / 2 {
                let i2 = inverted_output.1 - 1 - i;

                let temp = inverted_output.0[i];
                inverted_output.0[i] = inverted_output.0[i2];
                inverted_output.0[i2] = temp;

                i += 1;
            }

            inverted_output
        };

        (MaybeUninit::new(output.0), output.1)
    }
}
