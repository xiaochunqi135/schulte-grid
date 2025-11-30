mytext = open("./src/app.rs", "r").read()


unicode_list = [ord(c) for c in mytext]

unicode_list.sort()

output = list(dict.fromkeys(unicode_list))

print(output)
