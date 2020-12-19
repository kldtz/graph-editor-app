function response_to_json(response) {
    if (response.ok) {
        return response.json();
    } else {
        response.text().then(text => { throw Error(text); });
    }
}

class Graph {
    constructor(opts) {
        this.svg = opts.svg;
        this.graph_id = opts.graph_id;
        this.translate_x = opts.translate_x;
        this.translate_y = opts.translate_y;
        this.scale = opts.scale;
        this.setNodes(opts.nodes);
        this.setEdges(opts.edges);
        this.state = {
            mouseOverNode: null,
            shiftNodeDrag: false,
            selectedNode: null,
            selectedEdge: null,
            dragStartX: null,
            dragStartY: null,
        };
        this.consts = {
            BACKSPACE_KEY: 8,
            DELETE_KEY: 46,
            NODE_RADIUS: 50,
            CLICK_DISTANCE: 5,
            ENTER_KEY: 13,
        };
        this.draw();
    }

    setEdges(edges) {
        // map source and target id to respective node
        this.edges = edges.map(e => {
            return {
                id: e.id,
                source: this.nodes.find(n => n.id == e.source_id),
                target: this.nodes.find(n => n.id == e.target_id),
                label: e.edge_label
            }
        });
    }

    setNodes(nodes) {
        this.nodes = nodes.map(n => {
            return {
                id: n.id,
                title: n.node_label,
                x: n.x_coord,
                y: n.y_coord,
            }
        })
    }

    draw() {
        d3.select(window).on("keydown", (event) => {
            switch (event.keyCode) {
                case this.consts.BACKSPACE_KEY:
                case this.consts.DELETE_KEY:
                    if (this.state.selectedNode) {
                        event.preventDefault();
                        const selected = this.state.selectedNode;
                        fetch('/nodes/' + selected.id, {
                            method: 'DELETE',
                            headers: { 'Content-Type': 'application/json' },
                        })
                            .then(response_to_json)
                            .then(response => this.deleteNode(selected))
                            .catch(err => console.error(err));
                    } else if (this.state.selectedEdge) {
                        event.preventDefault();
                        const selected = this.state.selectedEdge;
                        fetch('/edges/' + selected.id, {
                            method: 'DELETE',
                            headers: { 'Content-Type': 'application/json' },
                        })
                            .then(response_to_json)
                            .then(response => this.deleteEdge(selected))
                            .catch(err => console.error(err));
                    }
                    break;
            }
        });

        // prepare SVG
        this.svg
            .on("mousedown", (event, d) => {
                if (event.shiftKey) {
                    const pos = d3.pointer(event, graph.plot.node())
                    fetch('/nodes', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ node_label: "new node", graph_id: this.graph_id, x_coord: pos[0], y_coord: pos[1] })
                    })
                        .then(response_to_json)
                        .then(node => this.addNode(node))
                        .catch(err => console.error(err));
                }
            })
            .on('click', () => {
                this.state.selectedNode = null;
                this.state.selectedEdge = null;
                this.update();
            });

        this.defineMarkers();

        // drag behavior
        const graph = this;
        this.drag = d3.drag()
            .clickDistance(this.consts.CLICK_DISTANCE)
            .on("drag", function (event, d) {
                if (graph.state.shiftNodeDrag) {
                    const pos = d3.pointer(event, graph.plot.node());
                    graph.dragLine.attr('d', 'M' + d.x + ',' + d.y + 'L' + pos[0] + ',' + pos[1]);
                } else {
                    d.x = event.x;
                    d.y = event.y;
                    d3.select(this).raise().attr("transform", d => "translate(" + [d.x, d.y] + ")");
                    graph.updateEdges();
                }
            })
            .on("start", (event, source) => {
                if (!this.state.shiftNodeDrag) {
                    this.state.dragStartX = source.x;
                    this.state.dragStartY = source.y;
                }
            })
            .on("end", (event, source) => {
                if (!this.state.shiftNodeDrag) {
                    fetch('/nodes/' + source.id, {
                        method: 'PATCH',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ x_coord: source.x, y_coord: source.y })
                    })
                        .then(response_to_json)
                        .catch(err => {
                            console.error(err);
                            // undo movement
                            source.x = this.state.dragStartX;
                            source.y = this.state.dragStartY;
                            this.update();
                        })
                }
                this.state.shiftNodeDrag = false;
                // hide line, remove arrow tip
                this.dragLine.classed("hidden", true);

                const target = this.state.mouseOverNode;

                if (!source || !target) return;

                // source and target are different
                if (source !== target) {
                    fetch('/edges', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ edge_label: "new edge", source_id: source.id, target_id: target.id })
                    })
                        .then(response_to_json)
                        .then(edge => this.addEdge(edge))
                        .catch(err => console.error(err));
                }
            });

        // populate svg
        this.plot = this.svg.append('g').attr('transform', "translate(" + this.translate_x + "," + this.translate_y + ") scale(" + this.scale + ")");


        // add zoom behavior to whole svg
        const zoom = d3.zoom()
            .clickDistance(this.consts.CLICK_DISTANCE)
            .on('zoom', (event) => {
                this.plot.attr('transform', event.transform);
            })
            .on('end', (event) => {
                const t = d3.zoomTransform(this.plot.node());
                fetch('/graphs/' + this.graph_id, {
                    method: 'PATCH',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ translate_x: t.x, translate_y: t.y, scale: t.k })
                })
            });

        const initZoom = d3.zoomIdentity.translate(this.translate_x, this.translate_y).scale(this.scale);

        this.svg.call(zoom.transform, initZoom)
            .call(zoom);

        // displayed when dragging between nodes
        this.dragLine = this.plot.append('path')
            .classed('line', true)
            .classed('dragline', true)
            .classed('hidden', true)
            .attr('d', 'M0,0L0,0');

        // circles need to be added last to be drawn above the paths
        this.paths = this.plot.append('g').classed('edges', true);
        this.circles = this.plot.append('g').classed('nodes', true);

        this.update();
    }

    addNode(node) {
        this.nodes.push({ id: node.id, title: node.node_label, x: node.x_coord, y: node.y_coord });
        this.updateNodes();
    }

    addEdge(edge) {
        this.edges.push({
            id: edge.id,
            label: edge.edge_label,
            source: this.nodes.find(n => n.id == edge.source_id),
            target: this.nodes.find(n => n.id == edge.target_id)
        });
        this.updateEdges();
    }

    deleteNode(selected) {
        this.nodes = this.nodes.filter(node => { return selected !== node; });
        this.edges = this.edges.filter(edge => { return edge.source !== selected && edge.target !== selected; });
        this.update();
    }

    deleteEdge(selected) {
        this.edges = this.edges.filter(edge => { return selected !== edge; });
        this.updateEdges();
    }

    defineMarkers() {
        const defs = this.svg.append('defs');
        // arrow marker for edge
        defs.append('marker')
            .attr('id', 'end-arrow')
            // keep same scale
            .attr('markerUnits', 'userSpaceOnUse')
            .attr('viewBox', '-20 -10 20 20')
            .attr('markerWidth', 20)
            .attr('markerHeight', 20)
            // tip of marker at circle (cut off part of tip that is thinner than line)
            .attr('refX', this.consts.NODE_RADIUS - 3)
            .attr('orient', 'auto')
            .append('path')
            .attr('d', 'M-20,-10L0,0L-20,10');
        // arrow marker for selected edge (to allow separate CSS styling)
        defs.append('marker')
            .attr('id', 'selected-end-arrow')
            // keep same scale
            .attr('markerUnits', 'userSpaceOnUse')
            .attr('viewBox', '-20 -10 20 20')
            .attr('markerWidth', 20)
            .attr('markerHeight', 20)
            // tip of marker at circle (cut off part of tip that is thinner than line)
            .attr('refX', this.consts.NODE_RADIUS - 3)
            .attr('orient', 'auto')
            .append('path')
            .attr('d', 'M-20,-10L0,0L-20,10');
        // arrow marker for leading arrow
        defs.append('marker')
            .attr('id', 'mark-end-arrow')
            // keep same scale
            .attr('markerUnits', 'userSpaceOnUse')
            .attr('viewBox', '-20 -10 20 20')
            .attr('markerWidth', 20)
            .attr('markerHeight', 20)
            // tip of marker at end of line
            .attr('refX', -5)
            .attr('orient', 'auto')
            .append('path')
            .attr('d', 'M-20,-10L0,0L-20,10');
    }

    update() {
        this.updateEdges();
        this.updateNodes();
    }

    updateNodes() {
        // enter node groups
        const nodes = this.circles.selectAll("g")
            .data(this.nodes, d => { return d.id; })
            .join(
                enter => {
                    const nodes = enter.append("g")
                        .attr("class", "node")
                        .attr("id", d => { return d.id; })
                        //.attr("data-motion", "click->select_node")
                        .attr("transform", d => { return "translate(" + d.x + "," + d.y + ")"; })
                        .on("mousedown", (event, d) => {
                            event.stopPropagation();
                            if (event.shiftKey) {
                                this.state.shiftNodeDrag = true;
                                this.dragLine.classed('hidden', false)
                                    .attr('d', 'M' + d.x + ',' + d.y + 'L' + d.x + ',' + d.y);
                            }
                        })
                        .on("mouseover", (event, d) => { this.state.mouseOverNode = d; })
                        .on("mouseout", () => { this.state.mouseOverNode = null; })
                        .on("click", (event, d) => {
                            event.stopPropagation();
                            if (event.shiftKey) {
                                this.editNodeLabel(d);
                            } else {
                                this.state.selectedNode = d;
                                this.state.selectedEdge = null;
                                this.update();
                            }
                        })
                        .call(this.drag);

                    nodes.append("circle")
                        .attr("r", String(this.consts.NODE_RADIUS));

                    nodes.append("text")
                        .attr("dy", 5)
                        .text(d => { return d.title; });
                },
                update => {
                    update.attr("transform", d => { return "translate(" + d.x + "," + d.y + ")"; })
                        .classed("selected", d => { return d === this.state.selectedNode; });

                    update.select("text")
                        .text(d => { return d.title; });
                },
                exit => exit.remove()
            );
    }

    editNodeLabel(d) {
        const selection = this.circles.selectAll('g').filter(function (dval) {
            return dval.id === d.id;
        });
        // hide current label
        const text = selection.selectAll("text").classed("hidden", true);
        // add intermediate editable paragraph
        const d3txt = this.plot.selectAll("foreignObject")
            .data([d])
            .enter()
            .append("foreignObject")
            .attr("x", d.x - this.consts.NODE_RADIUS)
            .attr("y", d.y - 13)
            .attr("height", 2 * this.consts.NODE_RADIUS)
            .attr("width", 2 * this.consts.NODE_RADIUS)
            .append("xhtml:div")
            .attr("id", "editable-p")
            .attr("contentEditable", "true")
            .style("text-align", "center")
            //.style("border", "1px solid")
            .text(d.title)
            .on("mousedown", (event, d) => {
                event.stopPropagation();
            })
            .on("keydown", (event, d) => {
                event.stopPropagation();
                if (event.keyCode == this.consts.ENTER_KEY) {
                    event.target.blur();
                }
            })
            .on("blur", (event, d) => {
                const new_label = event.target.textContent;
                fetch('/nodes/' + d.id, {
                    method: 'PATCH',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ node_label: new_label })
                })
                    .then(response_to_json)
                    .then(node => {
                        d.title = node.node_label;
                        d3.select(event.target.parentElement).remove();
                        this.updateNodes();
                        text.classed("hidden", false);
                    })
                    .catch(err => console.error(err));
            });
        d3txt.node().focus();
    }

    updateEdges() {
        this.paths.selectAll(".edge")
            .data(this.edges, this.edgeId)
            .join(
                enter => {
                    const edges = enter.append("g")
                        .attr("id", d => { return d.id; })
                        //.attr("data-motion", "click->select_edge")
                        .classed("edge", true)
                        .on("click", (event, d) => {
                            event.stopPropagation();
                            if (event.shiftKey) {
                                this.editEdgeLabel(d);
                            } else {
                                this.state.selectedEdge = d;
                                this.state.selectedNode = null;
                                this.update();
                            }
                        })
                        .on("mousedown", (event, d) => {
                            event.stopPropagation();
                        });

                    edges.append("path")
                        .attr("id", this.edgeId)
                        .classed("line", true)
                        .attr("d", d => {
                            return "M" + d.source.x + "," + d.source.y + "L" + d.target.x + "," + d.target.y;
                        });

                    edges.append("text")
                        .attr("class", "edge-label")
                        .attr("dy", - 10)
                        .attr("fill", "black")
                        .append("textPath")
                        .attr("xlink:href", d => "#" + this.edgeId(d))
                        .attr("text-anchor", "middle")
                        .attr("startOffset", "50%")
                        .text(d => d.label);
                },
                update => {
                    update.classed("selected", d => { return d === this.state.selectedEdge; });

                    update.select("path")
                        .attr("d", d => {
                            return "M" + d.source.x + "," + d.source.y + "L" + d.target.x + "," + d.target.y;
                        });

                    update.select("text").select("textPath").text(d => d.label);
                },
                exit => exit.remove()
            );
    }

    edgeId(d) {
        return String(d.source.id) + "+" + String(d.target.id);
    }

    editEdgeLabel(d) {
        const selection = this.paths.selectAll('g').filter(dval => {
            return this.edgeId(dval) === this.edgeId(d);
        });
        // hide current label
        const text = selection.selectAll("text").classed("hidden", true);
        // add intermediate editable paragraph
        const d3txt = this.plot.selectAll("foreignObject")
            .data([d])
            .enter()
            .append("foreignObject")
            .attr("x", d.target.x - (d.target.x - d.source.x) / 2)
            .attr("y", d.target.y - (d.target.y - d.source.y) / 2)
            .attr("height", 100)
            .attr("width", 100)
            .append("xhtml:div")
            //.style("transform", "rotate(20deg)")
            .attr("id", "editable-p")
            .attr("contentEditable", "true")
            .style("text-align", "center")
            //.style("border", "1px solid")
            .text(d.label)
            .on("mousedown", (event, d) => {
                event.stopPropagation();
            })
            .on("keydown", (event, d) => {
                event.stopPropagation();
                if (event.keyCode == this.consts.ENTER_KEY) {
                    event.target.blur();
                }
            })
            .on("blur", (event, d) => {
                const new_label = event.target.textContent;
                fetch('/edges/' + d.id, {
                    method: 'PATCH',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ edge_label: new_label })
                })
                    .then(response_to_json)
                    .then(edge => {
                        d.label = edge.edge_label;
                        d3.select(event.target.parentElement).remove();
                        this.updateEdges();
                        text.classed("hidden", false);
                    })
                    .catch(err => console.error(err));
            });
        d3txt.node().focus();
    }
}

/* Main */

const svg = d3.select("#graph");
const graph_id = svg.node().getAttribute("data-graph-id");

fetch("/graphs/find/" + graph_id)
    .then(response_to_json)
    .then(data => {
        console.log(data);
        const graph = new Graph({
            svg: svg,
            nodes: data.nodes,
            edges: data.edges,
            translate_x: data.graph.translate_x,
            translate_y: data.graph.translate_y,
            scale: data.graph.scale,
            graph_id: parseInt(graph_id),
        })
    });