/**
*
    MMSystem.h:437 -> 472
*
**/

use super::super::prelude::SoundPlayOption;



/**
*
//TODO
#ifdef _WIN32
#define sndAlias(ch0, ch1)      (SND_ALIAS_START + (DWORD)(BYTE)(ch0) | ((DWORD)(BYTE)(ch1) << 8))

#define SND_ALIAS_SYSTEMASTERISK        sndAlias('S', '*')
#define SND_ALIAS_SYSTEMQUESTION        sndAlias('S', '?')
#define SND_ALIAS_SYSTEMHAND            sndAlias('S', 'H')
#define SND_ALIAS_SYSTEMEXIT            sndAlias('S', 'E')
#define SND_ALIAS_SYSTEMSTART           sndAlias('S', 'S')
#define SND_ALIAS_SYSTEMWELCOME         sndAlias('S', 'W')
#define SND_ALIAS_SYSTEMEXCLAMATION     sndAlias('S', '!')
#define SND_ALIAS_SYSTEMDEFAULT         sndAlias('S', 'D')
*
**/

/**
    play synchronously (default) 
**/
pub static        Sync : SoundPlayOption = 0x0000;

/**
    play asynchronously 
**/
pub static       Async : SoundPlayOption = 0x0001;

/**
    silence (!default) if sound not found 
**/
pub static   NoDefault : SoundPlayOption = 0x0002;

/**
    pszSound points to a memory file 
**/
pub static      Memory : SoundPlayOption = 0x0004;

/**
    loop the sound until next sndPlaySound 
**/
pub static        Loop : SoundPlayOption = 0x0008;

/**
    don't stop any currently playing sound 
**/
pub static      NoStop : SoundPlayOption = 0x0010;

/**
    don't wait if the driver is busy 
**/
pub static      NoWait : SoundPlayOption = 0x00002000;

/**
    name is a registry alias 
**/
pub static       Alias : SoundPlayOption = 0x00010000;

/**
    alias is a predefined ID 
**/
pub static     AliasId : SoundPlayOption = 0x00110000;

/**
    name is file name 
**/
pub static    FileName : SoundPlayOption = 0x00020000;

/**
    name is resource name or atom 
**/
pub static    Resource : SoundPlayOption = 0x00040004;

/**
    purge non-static events for task 
**/
pub static       Purge : SoundPlayOption = 0x0040;

/**
    look for application specific association 
**/
pub static Application : SoundPlayOption = 0x0080;

/**
    Generate a SoundSentry event with this sound 
**/
pub static      Sentry : SoundPlayOption = 0x00080000;

/**
    Treat this as a "ring" from a communications app - don't duck me 
**/
pub static        Ring : SoundPlayOption = 0x00100000;

/**
    Treat this as a system sound 
**/
pub static      System : SoundPlayOption = 0x00200000;

/**
    alias base 
**/
pub static  AliasStart : SoundPlayOption = 0;