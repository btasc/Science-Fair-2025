from PIL import Image, ImageDraw
import json
import random

try:
    with open('network.json', 'r') as f:
        network = json.load(f)
except FileNotFoundError:
    print('network.json not found')
    exit()


nodes = []
connections = []

connection_node_app = {}

x_axis = 0
inc = 100

y_spread = list(range(0, len(network['nodes'])))

for layer in network['layers']:
    x_axis += inc

    for node in layer:
        random_index = random.randint(0, len(y_spread)-1)
        y_axis = y_spread.pop(random_index) * inc + inc

        connection_node_app[node] = (x_axis+10, y_axis+10)

        nodes.append((x_axis, y_axis))

for connection in network['connections']:
    connections.append((connection_node_app[connection[0]], connection_node_app[connection[1]]))

img = Image.new('RGB', (x_axis+inc, len(network['nodes']) * inc + inc), 'black')
draw = ImageDraw.Draw(img)

def node(x, y):
    draw.ellipse([x, y, x+20, y+20], fill='#596475', outline='#8cb6fa')

def line(node1, node2):
    draw.line([node1, node2], fill='#8cb6fa', width=3)

for node_coord in nodes:
    node(node_coord[0], node_coord[1])
for connection in connections:
    line(connection[0], connection[1])

img.save('test.png')