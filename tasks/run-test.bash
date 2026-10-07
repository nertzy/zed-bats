# Zed hands a task's command to the shell as source text, after replacing
# every ZED_ variable reference in it, so this script reads them with printenv.
file=$(printenv ZED_FILE) row=$(printenv ZED_ROW)
test_pattern='^[[:blank:]]*@test[[:blank:]]+(.*[^[:blank:]])[[:blank:]]+\{(.*)$'
function_pattern='[[:blank:]]*([^[:blank:]()]+)[[:blank:]]*\(?\)?[[:blank:]]+\{[[:blank:]]+#[[:blank:]]*@test[[:blank:]]*$'
current=0 name=
while ((current < row)) && IFS= read -r line; do
  ((current += 1))
  line=${line//$'\r'/}
  if [[ $line =~ $test_pattern || $line =~ $function_pattern ]]; then
    name=${BASH_REMATCH[1]#[\'\"]} name=${name%[\'\"]}
  fi
done < "$file"
if [[ -z $name ]]; then
  printf 'No Bats test starts on or above line %s of %s\n' "$row" "$file" >&2
  exit 1
fi
filter=
for ((i = 0; i < ${#name}; i++)); do
  char=${name:i:1}
  case $char in [][\\.*+?^\$\|\(\)\{\}]) filter+=\\ ;; esac
  filter+=$char
done
exec bats --filter "^$filter\$" "$file"
