#!/bin/bash

###########################################################################
# Download star data from Yale Bright Stars Catalog and Hipparcos Catalog #
# Yale: http://tdc-www.harvard.edu/catalogs/bsc5.html                     #
# Hipparcos: https://heasarc.gsfc.nasa.gov/W3Browse/all/hipparcos.html    #
###########################################################################


mkdir -p data/Hipparcos
mkdir -p data/Yale

(
    cd data/Hipparcos
    wget -cv 'https://cdsarc.cds.unistra.fr/ftp/cats/I/239/hip_main.dat'
    wget -cv 'https://cdsarc.cds.unistra.fr/ftp/cats/I/239/ReadMe'
)

(
    cd data/Yale
    wget -cv 'http://tdc-www.harvard.edu/catalogs/bsc5.dat.gz'
    wget -cv 'http://tdc-www.harvard.edu/catalogs/bsc5.readme'
    wget -cv 'http://tdc-www.harvard.edu/catalogs/bsc5.notes.gz'
    gunzip bsc5.dat.gz
    gunzip bsc5.notes.gz
)