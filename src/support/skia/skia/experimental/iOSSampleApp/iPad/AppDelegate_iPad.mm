#import "AppDelegate_iPad.h"

@implementation AppDelegate_iPad

@synthesize window, splitViewController;

- (BOOL)application:(UIApplication *)application didFinishLaunchingWithOptions:(NSDictionary *)launchOptions {
    [window addSubview:[splitViewController view]];
    [window makeKeyAndVisible];
    return YES;
}

- (void)dealloc {
    [window release];
    [splitViewController release];
    [super dealloc];
}

@end
