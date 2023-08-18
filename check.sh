for file in *.o; do
    nm "$file" | grep print_f64 && echo "$file"
done

