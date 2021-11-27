pub fn the_menu_yes_no() -> String {
    use terminal_menu::{button, label, menu, mut_menu, run};
    let menu = menu(vec![
        // label:
        //  not selectable, usefule as a title, separator, etc...
        label("----------------------"),
        label("Valid commands : YES, NO"),
        label("-----------------------"),
        // button:
        //  exit the menu
        button("Yes"),
        button("No"),
        button("Exit"),
    ]);
    run(&menu);

    // you can get the selected buttons name like so:
    let save = mut_menu(&menu).selected_item_name().to_string();
    print!("\x1B[2J\x1B[1;1H");
    //println!("Selected: {}", mut_menu(&menu).selected_item_name());
    save
}

pub fn the_menu_scan_search() -> String {
    use terminal_menu::{button, label, menu, mut_menu, run};
    let menu = menu(vec![
        // label:
        //  not selectable, usefule as a title, separator, etc...
        label("----------------------"),
        label("Valid commands : scan, search"),
        label("-----------------------"),
        // button:
        //  exit the menu
        button("scan"),
        button("search"),
    ]);
    run(&menu);

    // you can get the selected buttons name like so:
    let save = mut_menu(&menu).selected_item_name().to_string();
    print!("\x1B[2J\x1B[1;1H");
    //println!("Selected: {}", mut_menu(&menu).selected_item_name());
    save
}

pub fn the_menu_path() -> String {
    use terminal_menu::{button, label, menu, mut_menu, run};
    let menu = menu(vec![
        // label:
        //  not selectable, usefule as a title, separator, etc...
        label("----------------------"),
        label("PATH: "),
        label("-----------------------"),
        // button:
        //  exit the menu
        button("/Users/wissemcherifi/Desktop/testmusique"),
        button("Exit"),
    ]);
    run(&menu);

    // you can get the selected buttons name like so:
    let save = mut_menu(&menu).selected_item_name().to_string();
    print!("\x1B[2J\x1B[1;1H");
    // println!("Selected: {}", mut_menu(&menu).selected_item_name());
    save
}

pub fn the_menu_categorie() -> String {
    use terminal_menu::{button, label, menu, mut_menu, run};
    let menu = menu(vec![
        // label:
        //  not selectable, usefule as a title, separator, etc...
        label("----------------------"),
        label("CATEGORIES : "),
        label("-----------------------"),
        // button:
        //  exit the menu
        button("Artist"),
        button("Title"),
        button("Year"),
        button("Album"),
    ]);
    run(&menu);

    // you can get the selected buttons name like so:
    let save = mut_menu(&menu).selected_item_name().to_string();
    print!("\x1B[2J\x1B[1;1H");
    //println!("Selected: {}", mut_menu(&menu).selected_item_name());
    save
}

pub fn the_menu_categorie_affinage() -> String {
    use terminal_menu::{button, label, menu, mut_menu, run};
    let menu = menu(vec![
        // label:
        //  not selectable, usefule as a title, separator, etc...
        label("----------------------"),
        label("CATEGORIES : "),
        label("-----------------------"),
        // button:
        //  exit the menu
        button("Artist"),
        button("Title"),
        button("Year"),
        button("Album"),
        button("No"),
    ]);
    run(&menu);

    // you can get the selected buttons name like so:
    let save = mut_menu(&menu).selected_item_name().to_string();
    print!("\x1B[2J\x1B[1;1H");
    //println!("Selected: {}", mut_menu(&menu).selected_item_name());
    save
}
