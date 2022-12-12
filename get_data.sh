#!/bin/bash

###########################################################################
# Download star data from Yale Bright Stars Catalog and Hipparcos Catalog #
# Yale: http://tdc-www.harvard.edu/catalogs/bsc5.html                     #
# Hipparcos: https://heasarc.gsfc.nasa.gov/W3Browse/all/hipparcos.html    #
# Open Source BSC: https://github.com/johanley/star-catalog/              #
###########################################################################

# TODO: make a simple rust cli to replace this script

mkdir -p data/{Hipparcos,Yale,OSBSC}

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

(
    cd data/OSBSC
    wget -cv 'https://github.com/johanley/star-catalog/raw/master/catalogs/output/open-source-bsc/ReadMe.utf8'
    wget -cv 'https://github.com/johanley/star-catalog/raw/master/catalogs/output/open-source-bsc/os-bright-star-catalog-hip.utf8'
    wget -cv 'https://github.com/johanley/constellation-lines/raw/master/output/constellation-lines-hip.utf8'
)