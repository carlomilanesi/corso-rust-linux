counter=$1
while [ $counter != 0 ]
do
    echo -e -n "\r$counter  "
    sleep 1
    counter=$((counter-1))
done
echo -e "\rFatto!"
