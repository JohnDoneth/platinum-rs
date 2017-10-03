extern crate cocoa;

use cocoa::base::{selector, id, nil, NO};

use cocoa::foundation::{NSRect, NSPoint, NSSize, NSAutoreleasePool, NSProcessInfo, NSString};

use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyRegular, NSWindow,
                    NSTitledWindowMask, NSBackingStoreBuffered, NSClosableWindowMask,
                    NSResizableWindowMask, NSMiniaturizableWindowMask,
                    NSUnifiedTitleAndToolbarWindowMask, NSMenu, NSMenuItem, NSTabView,
                    NSTabViewItem, NSRunningApplication, NSApplicationActivateIgnoringOtherApps};

fn main() {
    unsafe {
        // create tab view
        let tab_view = NSTabView::new(nil)
            .initWithFrame_(NSRect::new(NSPoint::new(0., 0.), NSSize::new(800., 800.)));

        // create a tab view item
        let tab_view_item = NSTabViewItem::new(nil)
            .initWithIdentifier_(NSString::alloc(nil).init_str("Tab View"));

        tab_view_item.setLabel(NSString::alloc(nil).init_str("Tab View Item"));
        tab_view.addTabViewItem_(tab_view_item);
    }
}

unsafe fn create_window(title: id, content: id) -> id {
    let _pool = NSAutoreleasePool::new(nil);

    let app = NSApp();

    // create a new Menu Bar
    let menubar = NSMenu::new(nil).autorelease();
    let app_menu_item = NSMenuIteem::new(nil).autorelease();
    menubar.addItem_(app_menu_item);

    // app menu
    let app_menu = NSMenu::new(nil).autorelease();
    let exit_prefix = NSString::alloc(nil).init_str("Exit...");
    let exit_title = exit_prefix.stringByAppendingString_(NSProcessInfo::processInfo(nil).procesName());
    let exit_action = selector("terminate(or):");
    let exit_key = NSString::alloc(nil).init_str("e");
    let exit_item = NSMenuItem::alloc(nil)
        .initWithTitle_action_keyEquivalent_(exit_title, exit_action, exit_key)
        .autorelease();
    app_menu.addItem_(exit_item);
    app_menu_item.setSubmenu_(app_menu);

    // draw the window
    let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer(
        NSRect::new(NSPoint::new(0., 0.), NSSize::new(800., 800.)),
        NSTitledWindowMask |
        NSClosableWindowMask |
        NSResizableWindowMask |
        NSMiniaturizableWindowMask |
        NSUnifiedTitleAndToolbarWindowMask,
        NSBackingStoreBuffered,
        NO
    ).autorelease();
    window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.));
    window.center();

    window.setTitle_(title);
    // window.setTitle_("Test"); for testing
    window.makeKeyAndOrderFront_(nil);

    window.setContentView_(content);
    let current_app = NSRunningApplication::currentApplication(nil);
    current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);

    return app;
}