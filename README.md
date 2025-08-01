Willow is a UNIX file tree interpreter and visualiser for your terminal.

In more understandable language - Willow takes a path to a directory, and then scans your entire file tree at that level, as well as dropping to lower layers of your file tree if you so specify.

This allows you to get a fast visual of your file tree at multiple layers without needing to run `ls` in multiple different directories.

This program was inspired by the linux CLI tool 'tree'.

### Usage

The command to run the program looks like this:
`willow {path} {--names | --paths} {depth}`

For example, if you wanted to run willow in your current directory, only displaying
the names of files (rather than full paths), with a depth level of 3:
`willow ./ --names 3`

The above command will run willow from your current directory, printing every file within your file tree up to a depth of 3 layers.
