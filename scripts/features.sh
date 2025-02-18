group_features_impl () {
  local prefix=$1

  if [ "$#" -gt 1 ] && [ -n "$2" ]; then
    local cur=$2
    shift 2
    group_features_impl "$prefix" "$@"
    group_features_impl "$prefix$cur," "$@"
  else
    echo $prefix
  fi
}

group_features () {
  group_features_impl "" "$@"
}
