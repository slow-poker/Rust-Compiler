getFilenames() {
 curl https://github.com/danieltan1517/teh_tarik/tree/master/phase1/src/examples | grep -o "[a-zA-Z]*\.tt" | sort -u
}
FILENAMES=$(getFilenames)
for filename in $FILENAMES
do
	#echo $filename
	LINKS="https://raw.githubusercontent.com/danieltan1517/teh_tarik/refs/heads/master/phase1/src/examples/$filename"
	#echo $LINKS
	wget -q $LINKS -O $filename
	echo "Created $filename"
done
