#/usr/bin/env bash
# Renames move modules in $bench, by finding modules with the same name in stdlib but with different priority.
# Usually works
# Invoke from repo root

# Path to benchmarking.rs
bench=pallets/sp-mvm/src/benchmarking.rs

# Path to new stdlib
basedir=$(dirname ${bench})/../tests/benchmark_assets/stdlib/artifacts/modules

find_new() {
    modname=$(echo $1 | grep -Po "(?<=_)(.*)(?=\.)")
    found=$(ls ${basedir}/*_$modname.mv 2>/dev/null)
    [ "$found" ] || {
        echo "No replacement found for $1" >&2
        return
    }
    newname=$(basename $found)
    # echo $modname '|' $1 '->' $newname
    [ "$newname" != "$1" ] && echo "s|modules/$1|modules/${newname}|g"
}

for F in $(cat ${bench} | grep -Po '(?<=/)\d+_(.+).mv' | sort -u); do
    pat=$(find_new $F)
    [ $pat ] && {
        echo for $F replacing \'$pat\'
        sed -i ".bak" "$pat" $bench
    }
done

exit 0
