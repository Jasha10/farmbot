Suppose farmbot is tracking a stow package `~/stow/stowme` with files as follows:
```
~/
  stow/
    stowme/
      foo/
        bar/
          baz.txt
```
Suppose farmbot is aware that the stow package has been stowed in targe dir `~/dev/folder/` as follows:
```
~/
  dev/
    folder/
      foo/
      other_file.txt
      bar -> relative symlink to ~/stow/stowme/foo/bar/
        baz.txt
```
## User stories

### mv baz.txt up one level & rename it
As a user, suppose I perform the following actions:
```
cd ~/dev/folder/foo
mv baz.txt ./baz_renamed.txt
```
This will cause my `~/stow` folder to look like this:
```
~/
  stow/
    stowme/
      foo/
        bar/
```
and my ~/dev folder will look like this:
```
~/
  dev/
    folder/
      foo/
      other_file.txt
      bar -> relative symlink to ~/stow/stowme/foo/bar/
      baz_renamed.txt
```
I want farmbot to detect this change and offer to update my `~/stow` folder like this:
```
~/
  stow/
    stowme/
      foo/
        bar/
        baz_renamed.txt
```
and my `~/dev` folder like this:
```
~/
  dev/
    folder/
      foo/
      other_file.txt
      bar -> relative symlink to ~/stow/stowme/foo/bar/
      baz_renamed.txt -> relative symlink to ~/stow/stowme/foo/baz_renamed.txt
```
This should also work if the user does `mv baz.txt ./baz.txt` (not renaming the file).
It should work for various permutations of where baz.txt has been moved to.
Farmbot will also offer to undo the change that the user has made.

In the user gui / popup message, farmbot should present the above six code blocks 
possibly omitting the `other_file.txt`, which is not relevant) so that the user will
understand what farmbot has observed as well as what it intends to do.

It should also present a concrete plan showing what commands it will run to achieve the intended goal.

Farmbot should also offer to undo the change that the user has made, including a concrete plan to accomplish this.

### mv bar/
As a user, suppose I perform the following actions:
```
cd ~/dev/folder/foo
mkdir qux
mv bar qux/bar_renamed
```
This will not cause any changes to my `~/stow` folder, but it will cause changes to my `~/dev` folder:
```
~/
  dev/
    folder/
      foo/
      other_file.txt
      qux/
        bar_renamed -> broken relative symlink
        baz.txt
```
I want farmbot to detect this change and offer to update my `~/stow` folder like this:
```
~/
  stow/
    stowme/
      foo/
        qux/
          bar_renamed/
            baz.txt
```
and my `~/dev` folder like this:
```
~/
  dev/
    folder/
      foo/
      other_file.txt
      qux/
        bar_renamed -> relative symlink to ~/stow/stowme/foo/qux/bar_renamed
          baz.txt
```
This should also work if the user e.g. deletes the symlink to bar, moves it without changing its name, moves it to a parent folder, etc.

## Implementation / Design
- use the `notify` rust crate to watch for filesystem changes
- Keep track of two kinds of trees:
  - actual file system subtrees
  - "view" trees representing how the user "sees" the filesystem (materializing symlinks)
