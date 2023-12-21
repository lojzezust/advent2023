"""Converts the input into format for Graphviz."""

modules = []
with open('inputs/day20.txt') as file:
    for line in file:
        line = line.strip()
        spec, out = line.split(" -> ")
        out_conn = out.split(", ")
        t,name = None, spec
        if spec[0] in "%&":
            t,name = spec[0], spec[1:]
        modules.append((t, name, out_conn))

with open('inputs/day20_gv.txt', 'w') as file:
    file.write("digraph G {\n")
    ffs = []
    conjs = []
    for (t,name,out_conn) in modules:
        if t == '%':
            ffs.append(name)
        else:
            conjs.append(name)

        for out in out_conn:
            file.write("    " + name + " -> " + out + "\n")

    file.write("\n\n")
    file.write("    " + ",".join(ffs) + " [shape=box]\n")
    file.write("    " + ",".join(conjs) + " [shape=ellipse]\n")
    file.write("}")
