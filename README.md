### Custom VCS tool

### General info

The aim of this project is to implement the basic VCS tool from scratch. The tool can be best
described as basic clone of GIT.
Currently, the tool handles four basic commands:

* ``add``
* ``commit``
* ```init```
* ``status``

They all mirror the functionality of corresponding commands in GIT.

### Technologies

* Rust

##### Libraries:

* bincode
* glob
* serde
* serde_derive
* sha1

### Setup
In order to evaluate the functionality of the tool, you need to run the main function and issue 
following command: ```init -p C:/path/to/your/directory```. Currently, it's necessary to specify exact path
where the tool's directory will be created.
