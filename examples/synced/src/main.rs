use either::Either;
use frender::{prelude::*, synced_vec_to_elements, SyncedVec};
use hooks::{ShareValue, Signal, SignalHook};

struct Item {
    index: usize,
    value: u32,
    selected: bool,
}

struct Data {
    cur: u32,
    selected_index: Option<usize>,
    items: SyncedVec<Item>,
}

impl Data {
    fn new() -> Self {
        Self {
            cur: 10,
            selected_index: None,
            items: (0..10)
                .map(|n| Item {
                    index: n,
                    value: n as u32,
                    selected: false,
                })
                .collect(),
        }
    }

    fn prepend(&mut self) {
        let next = self.cur + 10;
        self.items.splice(
            0..0,
            (self.cur..next).map(|value| Item {
                index: 0,
                value,
                selected: false,
            }),
        );
        self.cur = next;

        if let Some(ref mut selected_index) = self.selected_index {
            *selected_index += 10;
        }

        self.update_indices();
    }

    fn append(&mut self) {
        let next = self.cur + 10;
        self.items.extend(
            (self.cur..next)
                .zip(self.items.len()..)
                .map(|(value, index)| Item {
                    index,
                    value,
                    selected: false,
                }),
        );
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

            let (a, b) = self.items.get2_mut(1, i).unwrap();
            std::mem::swap(&mut a.index, &mut b.index);

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

        self.items
            .as_mut_slice_range_and_mark_outdated(idx..)
            .iter_mut()
            .for_each(|item| item.index -= 1);
    }

    fn update_indices(&mut self) {
        self.items
            .as_mut_slice_and_mark_all_outdated()
            .iter_mut()
            .enumerate()
            .for_each(|(index, item)| item.index = index)
    }
}

#[component(main(get_dom_element = "frender-root"))]
fn Main() {
    // data's updates are not reactive here
    let data = hooks::use_mut_with(|| hooks::GenSignalHook::new(Data::new())).to_signal();

    // let data = hooks::use_gen_signal_with(Data::new); // data's updates are reactive here

    (
        cs::div.children((
            cs::button
                .on_click(move |_: &_| data.map_mut(Data::prepend))
                .children("Prepend"),
            cs::button
                .on_click(move |_: &_| data.map_mut(Data::append))
                .children("Append"),
            cs::button
                .on_click(move |_: &_| data.map_mut(Data::clear))
                .children("Clear"),
            cs::button
                .on_click(move |_: &_| data.map_mut(Data::swap))
                .children("Swap"),
        )),
        cs::pre.children(component_fn!(move || {
            h![data.use_signal()];

            (
                cs::code.children(("Item count = ", { data.map(|data| data.items.len()) })),
                "\n",
                cs::code.children(("Next Index = ", { data.map(|data| data.cur) })),
                "\n",
                cs::code.children({
                    data.map(|data| {
                        data.selected_index
                            .map_or(Either::Left("No Selection"), |idx| {
                                Either::Right((
                                    "Selected   = ",
                                    data.items[idx].value,
                                    " (index = ",
                                    { idx },
                                    ")",
                                ))
                            })
                    })
                }),
            )
        })),
        cs::table.children(cs::tbody.children((
            cs::tr.children((
                cs::th.children("Index"),
                cs::th.children("Value"),
                cs::th.children("Actions"),
            )),
            component_fn!(move || {
                h![data.use_signal()];

                data.to_element_with_fn(synced_vec_to_elements(
                    move |Data {
                              items,
                              //   selected_index,
                              ..
                          }: &Data| {
                        // let selected_index = *selected_index;
                        items.to_element_with_fn(move |item: &_| {
                            let Item {
                                index: idx,
                                value,
                                selected,
                            } = *item;

                            // let selected = selected_index == Some(idx);

                            cs::tr
                                .style(if selected {
                                    Some("outline: outset 1px orange")
                                } else {
                                    None
                                })
                                .children((
                                    cs::td.children(idx),
                                    cs::td.children(value),
                                    cs::td.children((
                                        cs::button
                                            .on_click(move |_: &_| {
                                                data.map_mut(|data| {
                                                    if data.selected_index == Some(idx) {
                                                        // unselect
                                                        data.items[idx].selected = false;
                                                        data.selected_index = None
                                                    } else {
                                                        // select
                                                        if let Some(old) = data.selected_index {
                                                            data.items[old].selected = false;
                                                        }
                                                        data.items[idx].selected = true;
                                                        data.selected_index = Some(idx)
                                                    }
                                                })
                                            })
                                            .children(if selected { "Unselect" } else { "Select" }),
                                        cs::button
                                            .on_click(move |_: &_| {
                                                data.map_mut(|data: &mut Data| data.remove(idx))
                                            })
                                            .children("Remove"),
                                    )),
                                ))
                        })
                    },
                ))
            }),
        ))),
    )
}
