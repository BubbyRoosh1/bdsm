# bdsm

### Bubby's Declarative System Manager
The abbreviation is a coincidence

### About
System manager thingy as seen in NixOS or GNU/Guix. The yaml formatted config file will populate /etc on system init.

I liked how declarative system configuration works, but I don't like NixOS/Guix, so this is going to work with as many distributions as possible.

Installation will move the system's init (/sbin/init), replace it with itself, and run the original init after it's run.

This is a smaller project to see if it could work maybe?

### TODO
* Install as mentioned in [about](#about)
* Add user command fix (edit /etc/passwd and /etc/shadow instead of system commands..?)
* More configs (locales for glibc, package manager/packages, etc)
