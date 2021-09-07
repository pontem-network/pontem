bench=pallets/sp-mvm/src/benchmarking.rs
basedir=$(dirname ${bench})/../tests/benchmark_assets/stdlib/artifacts/modules
find_new() {
    modname=$(echo $1 | grep -Po "(?<=_)(.*)(?=\.)")
    found=$(ls ${basedir}/*_$modname.mv)
    # echo "found $found for $1"
    newname=$(basename $found)
    # echo $modname '|' $1 '->' $newname
    [ $newname != $1 ] && echo "s|modules/$1|modules/${newname}|g"
}

for F in $(cat ${bench} | grep -Po '(?<=/)\d+_(.+).mv'); do
    pat=$(find_new $F)
    echo for $F replacing $pat
    sed -i "$pat" $bench
done
