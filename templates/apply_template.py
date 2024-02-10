take in one parameter, "template_name", and identify the corresponding folder in ./patterns/
take in another parameter, "crate_name"
calculate crate_name_pascal
that folder contains files corresponding to ../ 
for example, ./patterns/new_tool/Cargo.coml corresponds to ../Cargo.toml
(we want to use Pathlib)
for each file **/*.* in the template
    read the file
    if the file exists in the ../
        do the stuff
    apply the template,
etc