use either::Either;
use frender::prelude::*;
use hooks::ShareValue;

struct Data {
    cur: u32,
    selected_index: Option<usize>,
    items: Vec<u32>,
}

impl Data {
    fn new() -> Self {
        Self {
            cur: 10,
            selected_index: None,
            items: (0..10).collect::<Vec<u32>>(),
        }
    }

    fn prepend(&mut self) {
        let next = self.cur + 10;
        self.items.splice(0..0, self.cur..next);
        self.cur = next;

        if let Some(ref mut selected_index) = self.selected_index {
            *selected_index += 10;
        }
    }

    fn append(&mut self) {
        let next = self.cur + 10;
        self.items.extend(self.cur..next);
        self.cur = next;
    }

    fn clear(&mut self) {
        self.selected_index = None;
        self.items.clear();
    }

    fn swap(&mut self) {
        let len = self.items.len();
        if len >= 2 {
            let i = len - 2;
            self.items.swap(1, i);

            match &mut self.selected_index {
                Some(selected_index @ 1) => {
                    *selected_index = i;
                }
                Some(selected_index) if *selected_index == i => {
                    *selected_index = 1;
                }
                _ => {}
            }
        }
    }

    fn remove(&mut self, idx: usize) {
        if let Some(ref mut selected_index) = self.selected_index {
            if *selected_index == idx {
                self.selected_index = None;
            } else if *selected_index > idx {
                *selected_index -= 1;
            }
        }

        self.items.remove(idx);
    }
}

#[component(main(get_dom_element = "frender-root"), only_dom)]
fn Main() {
    let data = hooks::use_shared_state_with(Data::new);

    intrinsic!(
        div[[
            button.on_click(
                data.clone()
                    .into_callback_map_mut(callback!(Data::prepend, fn(&mut _)))
                    .accept_anything()
            )[["Prepend"]],
            button.on_click(
                data.clone()
                    .into_callback_map_mut(callback!(Data::append, fn(&mut _)))
                    .accept_anything()
            )[["Append"]],
            button.on_click(
                data.clone()
                    .into_callback_map_mut(callback!(Data::clear, fn(&mut _)))
                    .accept_anything()
            )[["Clear"]],
            button.on_click(
                data.clone()
                    .into_callback_map_mut(callback!(Data::swap, fn(&mut _)))
                    .accept_anything()
            )[["Swap"]],
        ]],
        pre[[
            code[["Item count = ", { data.map(|data| data.items.len()) }]],
            "\n",
            code[["Next Index = ", { data.map(|data| data.cur) }]],
            "\n",
            code[[{
                data.map(|data| {
                    data.selected_index
                        .map_or(Either::Left("No Selection"), |idx| {
                            Either::Right((
                                "Selected   = ",
                                data.items[idx],
                                " (index = ",
                                { idx },
                                ")",
                            ))
                        })
                })
            }]],
        ]],
        table[[tbody[[
            //
            tr[[th[["Index"]], th[["Value"]], th[["Actions"]]]],
            {
                data.map(
                    |Data {
                         items,
                         selected_index,
                         ..
                     }| {
                        items
                            .iter()
                            .enumerate()
                            .map(|(idx, &value)| {
                                let selected = *selected_index == Some(idx);

                                Keyed(
                                    value,
                                    intrinsic!(
                                        tr.style(if selected {
                                            Some("outline: outset 1px orange")
                                        } else {
                                            None
                                        })[[
                                            td[[{ idx }]],
                                            td[[{ value }]],
                                            td[[
                                                //
                                                button.on_click(
                                                    data.clone()
                                                        .into_callback_map_mut(callback!(
                                                            |data: &mut Data, idx| {
                                                                if data.selected_index == Some(idx)
                                                                {
                                                                    data.selected_index = None
                                                                } else {
                                                                    data.selected_index = Some(idx)
                                                                }
                                                            }
                                                        ))
                                                        .provide_last_argument_copied(idx)
                                                        .accept_anything()
                                                )[[{
                                                    if selected {
                                                        "Unselect"
                                                    } else {
                                                        "Select"
                                                    }
                                                }]],
                                                button.on_click(
                                                    data.clone()
                                                        .into_callback_map_mut(callback!(
                                                            |data: &mut Data, idx| data.remove(idx)
                                                        ))
                                                        .provide_last_argument_copied(idx)
                                                        .accept_anything()
                                                )[["Remove"]],
                                            ]],
                                        ]]
                                    ),
                                )
                            })
                            .collect::<Vec<_>>()
                    },
                )
            },
        ]]]],
    )
}
