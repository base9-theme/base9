a="string"
b="${variable_in_quote}string"
c='single_quote'
d=`backtick`

echo $a

if [[ -d dir ]]; then
  cat file_name
fi
