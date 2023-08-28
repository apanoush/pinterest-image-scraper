# pinterest image scraper

downloads (atm only a select amount) all the pins images from a given pinterest board and stores them locally in your computer

## usage

`./pinterest-image-scraper URL PATH`

where `URL` is your pinterest board url.
the program will then create a `images` (or `PATH` if given as argument) directory with all the pins images downloaded as `.jpg` files.

`PATH` is the name of the folder your photos will be downloaded in. it is **optional** and as said above the default name is `images`.