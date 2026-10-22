# PassRhythm
Rhythm based password manager!
This project aims at creating a password manager that uses rhythm patterns as the key, using KeePass software as the core password database.

# Creating a passrhythm
> Make sure you have installed `Quaver` first! It's free on Steam

1: Open Quaver and create a new chart<br>
2: Pick the song, chart settings, and chart your own passrhythm<br>
3: Memorize it, and playtest it rigorously!<br>
4: Launch the PassRhythm client, and select `Update master passrhythm`<br>
5: Select your chart file, located at `C:\Program Files (x86)\Steam\steamapps\common\Quaver\Songs\[your chart folder]\[your chart].qua`<br>
6: Perform the passrhythm 3 times in a row to make sure you can perform it, after that you will be prompted to confirm or cancel
- ⚠ To make sure the system is secure, confirming at this stage will permanently delete your chart file, if you want to keep it back it up, or draw the chart on a piece of paper

# Accessing your passwords from the PassRhythm client
1: Click `Unlock vault`<br>
2: Perform your passrhythm<br>
3: You're in!<br>

# How does it work?
### Creating the passrhythm
We compile a master password for the KBDX4 database using the data we can read from the chart file.
The data looks like this "6:L1,L2|24:L4|65:L3,L4", the first number is the progress of the chart, it depends on both time signature and bpm.
The second piece of information is the lane that note is on, if there are multiple they are sorted.
### Performing the passrhythm
We play the song and run the progress of the chart, then we attach your inputs (e.g. L1, L2) to the current progress.
We then attempt to unlock the vault with the results of the performance, if it works your performance was correct, otherwise it will retry.
