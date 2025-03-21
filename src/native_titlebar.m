// native_titlebar.m
#import <Cocoa/Cocoa.h>

void setup_titlebar_buttons(NSWindow *nsWindow) {
    // 1) Make the title bar transparent and extend the content view
    //    to cover the entire window, including the title bar region.
    [nsWindow setTitlebarAppearsTransparent:YES];
    nsWindow.styleMask |= NSWindowStyleMaskFullSizeContentView;

    // 2) Let the user drag the window by grabbing anywhere in the background.
    [nsWindow setMovableByWindowBackground:YES];

    // 3) Access the window's content view (which now extends into the title bar area).
    NSView *contentView = [nsWindow contentView];

    // 5) Position the button near the top-left corner (for example).
    //    Here, we set a fixed frame: (x=20, y=contentView height - 40, width=80, height=24).
    //    Adjust to your preference.
    NSRect contentFrame = [contentView frame];
    CGFloat buttonWidth = 80.0;
    CGFloat buttonHeight = 24.0;
    CGFloat margin = 8.0; // top margin

}

// Define the action method for the button press
void customButtonPressed(id sender) {
    (void)sender; // Silence unused-parameter warning if needed
    NSLog(@"Button Pressed!");
}
