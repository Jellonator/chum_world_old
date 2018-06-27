use gtk::{self, Widget, Container, Label, ScrolledWindow, Paned, ListBox};
use gtk::prelude::*;
use super::page::{Page, ArchiveFile};
use std::rc::Rc;
use std::cell::RefCell;
use super::plugin;
use util;
use std::fs;
use std::io::{Read, Write};

/// Creates an editor pane for editing the given ArchiveFile.
pub fn construct_editor(parent: Rc<RefCell<Page>>, file: Rc<RefCell<ArchiveFile>>, id: i32) -> Widget {
    let hbox = gtk::Box::new(gtk::Orientation::Vertical, 4);
    let grid = gtk::Grid::new();
    // Create grid layout
    grid.set_margin_left(4);
    grid.set_margin_right(4);
    grid.attach(&gtk::Label::new("Name"),    0, 0, 1, 1);
    grid.attach(&gtk::Label::new("Type"),    0, 1, 1, 1);
    grid.attach(&gtk::Label::new("Subtype"), 0, 2, 1, 1);
    grid.set_row_spacing(4);
    grid.set_column_spacing(4);
    // create entries
    let entry_name = gtk::Entry::new();
    entry_name.set_hexpand(true);
    entry_name.set_text(&file.borrow().name);
    let entry_type = gtk::Entry::new();
    entry_type.set_hexpand(true);
    entry_type.set_text(&file.borrow().typeid);
    let entry_subtype = gtk::Entry::new();
    entry_subtype.set_hexpand(true);
    entry_subtype.set_text(&file.borrow().subtypeid);
    grid.attach(&entry_name,    1, 0, 1, 1);
    grid.attach(&entry_type,    1, 1, 1, 1);
    grid.attach(&entry_subtype, 1, 2, 1, 1);
    hbox.add(&grid);
    // Connect entries to change file name
    let fname = Rc::downgrade(&file);
    let pname = Rc::downgrade(&parent);
    entry_name.connect_changed(move |s|{
        let pname = pname.upgrade().unwrap();
        let fname = fname.upgrade().unwrap();
        fname.borrow_mut().name = s.get_text().unwrap();
        pname.borrow_mut().set_file_name(id, &s.get_text().unwrap());
    });
    let ftype = Rc::downgrade(&file);
    entry_type.connect_changed(move |s|{
        let ftype = ftype.upgrade().unwrap();
        ftype.borrow_mut().typeid = s.get_text().unwrap();
    });
    let fsubtype = Rc::downgrade(&file);
    entry_subtype.connect_changed(move |s|{
        let fsubtype = fsubtype.upgrade().unwrap();
        fsubtype.borrow_mut().subtypeid = s.get_text().unwrap();
    });
    // create buttons
    let buttonbox = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    grid.attach(&buttonbox, 1, 3, 1, 1);
    let extract_button = gtk::Button::new_with_label("Extract");
    buttonbox.add(&extract_button);
    let replace_button = gtk::Button::new_with_label("Replace");
    buttonbox.add(&replace_button);
    // Connect buttons to functionality
    let fextract = Rc::downgrade(&file);
    let pextract = Rc::downgrade(&parent);
    extract_button.connect_clicked(move |_| {
        let pextract = pextract.upgrade().unwrap();
        let fextract = fextract.upgrade().unwrap();
        let app = &pextract.borrow().parent.upgrade().unwrap();
        let window = app.borrow().window.clone();
        let path = pextract.borrow().paths.d.parent().unwrap().to_owned();
        if let Some(path) = util::open_any(&path, "Extract file", &window, gtk::FileChooserAction::Save) {
            let mut fh = fs::File::create(path).unwrap();
            fh.write_all(&fextract.borrow().data).unwrap();
        }
    });
    let freplace = Rc::downgrade(&file);
    let preplace = Rc::downgrade(&parent);
    replace_button.connect_clicked(move |_| {
        let preplace = preplace.upgrade().unwrap();
        let freplace = freplace.upgrade().unwrap();
        let app = &preplace.borrow().parent.upgrade().unwrap();
        let window = app.borrow().window.clone();
        let path = preplace.borrow().paths.d.parent().unwrap().to_owned();
        if let Some(path) = util::open_any(&path, "Open file", &window, gtk::FileChooserAction::Open) {
            let mut fh = fs::File::open(path).unwrap();
            let mut newvec = Vec::new();
            fh.read_to_end(&mut newvec).unwrap();
            freplace.borrow_mut().data = newvec;
            Page::reset_file_editor(&preplace);
        }
    });
    // Add editor plugin
    hbox.add(&plugin::create_plugin_for_type(&file));
    // Return as a widget
    hbox.upcast::<Widget>()
}
