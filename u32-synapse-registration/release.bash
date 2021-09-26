#!/bin/bash

APP="u32-synapse-registration"
# the actual compiled binary
binary="$PWD/target/release/$APP";
echo "[$0] INFO: binary=$binary"
echo "[$0] INFO: $(ls -l --block-size=MB "$binary")"

CARGO_LOC="$PWD/Cargo.toml"
# get the version of this application from `$CARGO_LOC`
version="$(grep 'version' "$CARGO_LOC" | head -n 1 | sed 's| ||g; s|=||; s|version||; s|"||g;')"
echo "[$0] INFO: version=$version"
# compile a release build
cargo build --release

# make if not exists
releases="$PWD/releases"
echo "[$0] INFO: releases=$releases"
mkdir -v "$releases"
# release_tag should look like 0.0.1-MjAyMVNlcDI1MTE6MDY6NTcK
release_tag="$version-$(date | awk '{ print $4 $3 $2 $5 }' | base64)"
echo "[$0] INFO: release_tag=$release_tag"
# create a custom release dir based off the tag/s
release_dir="$releases/$release_tag";
echo "[$0] INFO: release_dir=$release_dir"
mkdir -v "$release_dir";
# copy the binary
cp -v "$binary" "$release_dir/$APP"

# copy the static release files
mkdir -v "$release_dir/static";
mkdir -v "$release_dir/static/css";
cp -v "$PWD"/static/css/*.css "$release_dir/static/css/"

# create a tar.gz release
cd "$releases" || exit
echo "[$0] INFO: current dir is $release_dir"

app_tar="$APP.tar.gz"
echo "[$0] INFO: app_tar=$app_tar"
echo "[$0] INFO: release_tag=$(ls "$release_tag")"

tar -cvzf "$app_tar" "$release_tag"
echo "[$0] INFO: $(ls -l --block-size=MB "$app_tar")"