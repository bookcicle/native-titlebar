// native_titlebar.m
#import <Cocoa/Cocoa.h>

void setup_titlebar_buttons(NSWindow *nsWindow) {
    NSButton *customButton = [NSButton buttonWithTitle:@"My Button"
                                                target:nil
                                                action:@selector(customButtonPressed:)];
    [customButton setFrame:NSMakeRect(60, nsWindow.frame.size.height - 30, 80, 20)];
    [customButton setBezelStyle:NSBezelStyleRounded];

    [nsWindow.contentView addSubview:customButton];
}

// Define action
void customButtonPressed(id sender) {
    NSLog(@"Button Pressed!");
}
