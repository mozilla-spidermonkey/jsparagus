(function (lib, img, cjs, ss, an) {

var p; // shortcut to reference prototypes
lib.webFontTxtInst = {}; 
var loadedTypekitCount = 0;
var loadedGoogleCount = 0;
var gFontsUpdateCacheList = [];
var tFontsUpdateCacheList = [];
lib.ssMetadata = [];



lib.updateListCache = function (cacheList) {		
	for(var i = 0; i < cacheList.length; i++) {		
		if(cacheList[i].cacheCanvas)		
			cacheList[i].updateCache();		
	}		
};		

lib.addElementsToCache = function (textInst, cacheList) {		
	var cur = textInst;		
	while(cur != exportRoot) {		
		if(cacheList.indexOf(cur) != -1)		
			break;		
		cur = cur.parent;		
	}		
	if(cur != exportRoot) {		
		var cur2 = textInst;		
		var index = cacheList.indexOf(cur);		
		while(cur2 != cur) {		
			cacheList.splice(index, 0, cur2);		
			cur2 = cur2.parent;		
			index++;		
		}		
	}		
	else {		
		cur = textInst;		
		while(cur != exportRoot) {		
			cacheList.push(cur);		
			cur = cur.parent;		
		}		
	}		
};		

lib.gfontAvailable = function(family, totalGoogleCount) {		
	lib.properties.webfonts[family] = true;		
	var txtInst = lib.webFontTxtInst && lib.webFontTxtInst[family] || [];		
	for(var f = 0; f < txtInst.length; ++f)		
		lib.addElementsToCache(txtInst[f], gFontsUpdateCacheList);		

	loadedGoogleCount++;		
	if(loadedGoogleCount == totalGoogleCount) {		
		lib.updateListCache(gFontsUpdateCacheList);		
	}		
};		

lib.tfontAvailable = function(family, totalTypekitCount) {		
	lib.properties.webfonts[family] = true;		
	var txtInst = lib.webFontTxtInst && lib.webFontTxtInst[family] || [];		
	for(var f = 0; f < txtInst.length; ++f)		
		lib.addElementsToCache(txtInst[f], tFontsUpdateCacheList);		

	loadedTypekitCount++;		
	if(loadedTypekitCount == totalTypekitCount) {		
		lib.updateListCache(tFontsUpdateCacheList);		
	}		
};
// symbols:



(lib.back = function() {
	this.initialize(img.back);
}).prototype = p = new cjs.Bitmap();
p.nominalBounds = new cjs.Rectangle(0,0,307,400);


(lib.btmin = function() {
	this.initialize(img.btmin);
}).prototype = p = new cjs.Bitmap();
p.nominalBounds = new cjs.Rectangle(0,0,257,79);


(lib.housemin = function() {
	this.initialize(img.housemin);
}).prototype = p = new cjs.Bitmap();
p.nominalBounds = new cjs.Rectangle(0,0,257,200);


(lib.logomin = function() {
	this.initialize(img.logomin);
}).prototype = p = new cjs.Bitmap();
p.nominalBounds = new cjs.Rectangle(0,0,200,78);


(lib.mg = function() {
	this.initialize(img.mg);
}).prototype = p = new cjs.Bitmap();
p.nominalBounds = new cjs.Rectangle(0,0,175,98);


(lib.nammin = function() {
	this.initialize(img.nammin);
}).prototype = p = new cjs.Bitmap();
p.nominalBounds = new cjs.Rectangle(0,0,94,145);// helper functions:

function mc_symbol_clone() {
	var clone = this._cloneProps(new this.constructor(this.mode, this.startPosition, this.loop));
	clone.gotoAndStop(this.currentFrame);
	clone.paused = this.paused;
	clone.framerate = this.framerate;
	return clone;
}

function getMCSymbolPrototype(symbol, nominalBounds, frameBounds) {
	var prototype = cjs.extend(symbol, cjs.MovieClip);
	prototype.clone = mc_symbol_clone;
	prototype.nominalBounds = nominalBounds;
	prototype.frameBounds = frameBounds;
	return prototype;
	}


(lib.Symbol39 = function(mode,startPosition,loop) {
	this.initialize(mode,startPosition,loop,{});

	// Layer 1
	this.shape = new cjs.Shape();
	this.shape.graphics.f("#000000").s().p("Ag7C8QgJgOgIgIQAWAIAFAAQgIgQgRAAQgKAAgYADQgSAAgFgGQgDgEgCgPIACAAIAAgFIgDgCIABAHIgLAAIgRgQQgIgIAAgCQAAgTAegGQAcgHAAgIQAAgIgKgEQgLgEgOACQABgIAJgGQAIgGAHAAQADAAAFADQAEACADAAIABgDIACgFQAAgGgVgLQgTgKgLAAQgHAAgHAGQgGAFABAEQgCgJgMgKIgMgHIAKgNQAKgKAKADQgEgKgOgFIgVgIQAEgNANACQAQAEAJAAIAAgSQgcgMgFAAQACgEAMgCIATgEQAAgBAAAAQAAgBABAAQAAAAAAgBQABAAABAAQgDgEgHgGQgFgEgBgDQABgJAHgGQAHgFAHAAQAJAAAHAPIADAAIAAgNQgGgFAAgFQABgEAHgGQAGgFAEAAQAEAAAKAHIABABIAAgDIAFgHQAEgDACAAQAEAAAFAFQAFAFAFAAIAVgHQAHAAAFAEQAEADAFAAQAFAAAGgHQAEgHAAgGQAAgFgCgCQgCgEgKgGQAHgIAJAAQALAAAOAOQAKAKAPgJQADgCAAAGQAAADgDAGQgCAFAAACIAPAAIAAASIAWAAQAFgEAJgDQAHgEgEgMIASAAQAAAEAEAMQAEALAAADIAWAAQgCAOAGAIQABADAMANQAKALAbgBQAVgBAAAPQAAATgYABQgYABAAAOQABAIAFACIAMABQgCAHgFAGQgGAEABAEQAAADAbAOQAcAPgBARQABAOgWAAQgLAAgMgFIgKgFIgGAKIAAAMQAHAAAGAGQAFAHAAAGQAAAIgLAEIgTAHQAAACACAEQADAEAAAGQAAAKgFAEQgFAEAAAMQAAAGAFAPIgSAAIgqgPQgEAAgIAKQgHAKgBAAIgMgFQgKgFgFAAQgHAAgGAKQgGAKgHAAQgSABgSAGQgQAGgEAAQgFAAgFgLg");

	this.timeline.addTween(cjs.Tween.get(this.shape).wait(1));

}).prototype = getMCSymbolPrototype(lib.Symbol39, new cjs.Rectangle(-21.2,-19.8,42.5,39.8), null);


(lib.Symbol27 = function(mode,startPosition,loop) {
	this.initialize(mode,startPosition,loop,{});

	// Layer 1
	this.shape = new cjs.Shape();
	this.shape.graphics.f("#FFFF00").s().p("AkdPqIAA/TII7AAIAAfTg");
	this.shape.setTransform(47.3,0,0.309,1);

	this.shape_1 = new cjs.Shape();
	this.shape_1.graphics.f("#FFFF00").s().p("AkdPqIAA/TII7AAIAAfTg");
	this.shape_1.setTransform(17.6,0,0.309,1);

	this.shape_2 = new cjs.Shape();
	this.shape_2.graphics.f("#FFFF00").s().p("AkdPqIAA/TII7AAIAAfTg");
	this.shape_2.setTransform(-27.5,0);

	this.timeline.addTween(cjs.Tween.get({}).to({state:[{t:this.shape_2},{t:this.shape_1},{t:this.shape}]}).wait(1));

}).prototype = getMCSymbolPrototype(lib.Symbol27, new cjs.Rectangle(-56.1,-100.1,112.3,200.4), null);


(lib.Symbol26 = function(mode,startPosition,loop) {
	this.initialize(mode,startPosition,loop,{});

	// Layer 1
	this.instance = new lib.mg();
	this.instance.parent = this;
	this.instance.setTransform(-87.7,96.5);

	this.instance_1 = new lib.mg();
	this.instance_1.parent = this;
	this.instance_1.setTransform(-87.7,193.5);

	this.instance_2 = new lib.mg();
	this.instance_2.parent = this;
	this.instance_2.setTransform(-87.7,-291.5);

	this.instance_3 = new lib.mg();
	this.instance_3.parent = this;
	this.instance_3.setTransform(-87.7,-195.5);

	this.instance_4 = new lib.mg();
	this.instance_4.parent = this;
	this.instance_4.setTransform(-87.7,-98.5);

	this.instance_5 = new lib.mg();
	this.instance_5.parent = this;
	this.instance_5.setTransform(-87.2,-1.5);

	this.timeline.addTween(cjs.Tween.get({}).to({state:[{t:this.instance_5},{t:this.instance_4},{t:this.instance_3},{t:this.instance_2},{t:this.instance_1},{t:this.instance}]}).wait(1));

}).prototype = getMCSymbolPrototype(lib.Symbol26, new cjs.Rectangle(-87.7,-291.5,175.5,583), null);


(lib.Symbol25 = function(mode,startPosition,loop) {
	this.initialize(mode,startPosition,loop,{});

	// Layer 1
	this.shape = new cjs.Shape();
	this.shape.graphics.f("#FFFFFF").s().p("ABPCPQgGgBgHgDQgFgDgLgPQgNgRgSgfIgbgtQgMgNgJgGQgMgFgPAAIgBAAIAABQQAAAWADAKQADALAHADQAKAEAQAAQAAAAAAAAQABABAAAAQAAABAAAAQAAABAAABQAAABAAABQAAABAAAAQAAABgBAAQAAAAAAABIiSAAQgBgBAAAAQAAAAgBgBQAAAAAAgBQAAgBAAgBQAAgBAAgBQAAAAAAgBQABAAAAgBQAAAAABAAQAQAAAKgEQAIgDADgLQACgIAAgYIAAigQABgYgDgIQgDgLgIgEQgIgDgRAAQAAAAgBAAQAAAAAAgBQAAgBAAAAQAAgBAAgBQAAgBAAgBQAAgBAAAAQAAgBAAAAQABgBAAAAICZAAQAAAAABABQAAAAAAABQAAAAAAABQAAABABABQgBABAAABQAAAAAAABQAAABAAAAQgBAAAAAAQgTAAgMADQgJAFgDAKQgEAMAAAVIAABEIAigCQAPABAQgnQAdg4ATgUQAVgTAYAAQAVAAAKAKQALAKAAAPQAAARgKAKQgJAKgSAAQgLAAgMgFQgKgGgLAAQgLAAgHAIQgIAHgMAYQgLAWgLAMQAVACANAGQANAHALAMQAMAOARAaIAQAZQAPAcAMALQALALAWAAQABAAAAAAQAAABAAAAQABABAAAAQAAABAAABQAAABAAABQAAABgBAAQAAABAAAAQAAAAgBABg");
	this.shape.setTransform(85,-0.7,0.413,0.413);

	this.shape_1 = new cjs.Shape();
	this.shape_1.graphics.f("#FFFFFF").s().p("AhSB/QgigUgSgjQgSgiAAgoQAAgvAYghQAZghAjgQQAlgQAmAAQAsgBAiAWQAhAUARAjQASAhAAAmQAAAsgVAiQgWAhglASQgkAUgpAAQgtAAghgWgAhKgdQAAAvAMAlQANAmAVAUQATAUAaAAQAcAAAOgbQAPgcAAg0QAAgxgNglQgNgjgVgTQgVgTgXAAQg5AAAABog");
	this.shape_1.setTransform(70.3,-0.5,0.413,0.413);

	this.shape_2 = new cjs.Shape();
	this.shape_2.graphics.f("#FFFFFF").s().p("ABRCKIgaABQgBAAAAAAQAAgBgBAAQAAgBAAgBQAAAAAAgCQAAAAAAgBQAAgBAAAAQABgBAAAAQAAAAABAAQARgBAHgGQAHgIAAgUIgOi2IhSDRQgBACgFAAQgDAAgCgCIhkjLIgICQQgBAgAMASQAMARAXAAQAAAAAAAAQABAAAAABQAAAAAAABQABABAAAAQAAACgBAAQAAABAAABQAAAAgBABQAAAAAAAAIgYgBIg8AAIgYABQgBAAAAAAQAAgBAAAAQgBgBAAgBQAAAAAAgCQAAAAAAgBQAAgBABAAQAAgBAAAAQAAAAABAAQAsAAAEhDIAJioQgLgPgJgFQgLgGgNABQgBAAAAgBQAAAAgBAAQAAgBAAgBQAAgBAAgBQAAgBAAAAQAAgBAAgBQABAAAAgBQAAAAABAAIBKAAQARAAAIAGQALAIALAXIBCCJIA+icQACgIAHgGQAGgEAGAAIBZAAQAAAAABAAQAAABAAAAQAAABAAABQAAAAAAABQAAABAAABQAAABAAABQAAAAAAAAQgBABAAAAQgXAAgLAKQgLAJACAcIAOCiQACAZADAJQACAJAIADQAIAEARABQABAAAAAAQAAAAABABQAAAAAAABQAAABAAAAQAAACAAAAQAAABAAABQgBAAAAABQAAAAgBAAg");
	this.shape_2.setTransform(54.5,-0.5,0.413,0.413);

	this.shape_3 = new cjs.Shape();
	this.shape_3.graphics.f("#FFFFFF").s().p("AAiCGQgLgLgEgSIggASQgQAIgXAIQgUAFgQABQgXAAgNgNQgOgMAAgWQAAgQAJgLQAIgJAPgHQAOgHAkgLIBIgYIAAg4QAAgrgJgVQgHgVgUAAQgRABgKANQgKAMgCAeQgFApgmAAQgigBAAgdQAAgTAZgUQAYgUAjgMQAjgMAYAAQAiAAAWAUQAVAVAAAmIAAB+QAAAWAHALQAIALAOAAQAKAAAKgEIABAAQADgBABAEQABABAAAAQAAABgBABQAAAAAAABQgBAAgBABIhKAiQgDACgEAAQgLAAgKgLgAgWAoQgTAGgJAJQgIAJAAAMQAAAQAIAIQAJAJANAAQAPAAAUgLIAKgEIgBhFg");
	this.shape_3.setTransform(40.4,-0.5,0.413,0.413);

	this.shape_4 = new cjs.Shape();
	this.shape_4.graphics.f("#FFFFFF").s().p("AhlCHQgFgDAAgCQgCgCgBgGIgFhDQAAgBAAgBQAAAAAAgBQABAAAAAAQABAAABgBQADAAACADQAPAlAYAXQAaAWAfABQAYgBALgQQAMgRABgeQgBgngSgTQgTgTgqAAQgDAAgCgDQAAgFAFAAQAmAAAQgOQAQgOAAgfQAAgagLgQQgLgQgTAAQgXAAgVAWQgTATgUAlIgFABQgFAAABgDIAQhgQAAAAAAgBQAAAAAAAAQABAAAAAAQABAAABAAIADABQABAAABAAQAAAAAAAAQAAABAAAAQAAAAAAAAIgBAIQAAAMAMgBQAIAAANgEQAdgKAfAAQAqAAAYAOQAZAPAAAdQAAAZgXATQgVATgnAKQAuADAZAVQAZAXAAAiQABAngkAWQgjAWg4gBQg6AAgggQg");
	this.shape_4.setTransform(27.8,-0.7,0.413,0.413);

	this.shape_5 = new cjs.Shape();
	this.shape_5.graphics.f("#FFFFFF").s().p("AAYDZQAAAAgBgBQAAAAAAAAQgBgBAAgBQAAgBAAgBQAAgBAAAAQAAgBABgBQAAAAAAAAQABgBAAAAQARAAAJgEQAIgDADgLQADgIAAgYIAAiDIh+B/IAAAEQAAAWADAKQADALAIADQAJAEAQAAQABAAAAABQAAAAAAAAQABABAAABQAAAAAAABQAAABAAABQAAABgBABQAAAAAAAAQAAABgBAAIiRAAQgBAAAAgBQgBAAAAAAQAAgBAAgBQAAgBAAgBQAAgBAAAAQAAgBAAgBQAAAAABAAQAAgBABAAQAQAAAJgEQAJgDACgLQADgKAAgWIAAigQAAgWgDgKQgCgKgJgEQgJgEgQAAQAAAAgBAAQAAAAAAgBQAAAAgBgBQAAgBAAgBQAAgBAAgBQABgBAAAAQAAgBAAAAQABAAAAAAICRAAQABAAAAAAQAAAAAAABQABAAAAABQAAABAAABQAAABAAABQAAABgBAAQAAABAAAAQAAAAgBAAQgQAAgJAFQgIAEgDAKQgDALAAAVIAACBIB+h/IAAgDQAAgYgDgIQgDgKgIgEQgKgEgPAAQAAAAgBAAQAAAAgBgBQAAAAAAgBQAAgBAAgBQAAgEACAAICRAAQAAAAAAAAQABAAAAABQAAAAAAABQAAABAAABQAAABAAABQAAABAAAAQAAABgBAAQAAAAAAAAQgQAAgJAFQgIAEgDAKQgDALAAAVIAACfQAAAWADAKQADALAIADQAIAEARAAQAAAAAAABQABAAAAAAQAAABAAABQAAAAAAABQAAABAAABQAAABAAABQAAAAgBAAQAAABAAAAgAgziBQgagLgOgOQgPgQAAgSQAAgNAIgHQAJgIAOAAQAOAAAHAGQAHAGADAJIAFAYQADATAIAKQAHAKAVAAQAWAAAJgLQAJgLADgUQAEgVAHgLQAGgKARAAQAOAAAIAIQAIAHAAANQAAATgPAPQgPAQgZAJQgYAJgbAAQgbAAgZgJg");
	this.shape_5.setTransform(6.9,-3.7,0.413,0.413);

	this.shape_6 = new cjs.Shape();
	this.shape_6.graphics.f("#FFFFFF").s().p("AhSB/QghgTgTgkQgSgiAAgoQAAgvAXghQAZghAkgQQAlgQAmAAQAtgBAgAWQAiAUARAjQASAiAAAlQAAArgWAjQgVAgglATQglAUgpAAQgsAAghgWgAhLgdQAAAwANAkQAMAlAVAVQAVAUAYAAQAcAAAPgbQAOgcAAg0QABgzgNgjQgNgjgVgTQgVgTgXAAQg6AAAABog");
	this.shape_6.setTransform(-8.3,-0.5,0.413,0.413);

	this.shape_7 = new cjs.Shape();
	this.shape_7.graphics.f("#FFFFFF").s().p("AifDzQAAAAgBgBQAAAAAAAAQgBgBAAgBQAAgBAAgBQAAgBAAAAQAAgBABgBQAAAAAAAAQABgBAAAAQAQAAAKgEQAIgEADgKQAEgLAAgVIAAlmQAAgegLgKQgLgKgaAAQAAAAgBAAQAAAAAAgBQAAAAAAgBQgBgBAAgBQAAgBABgBQAAgBAAAAQAAgBAAAAQABAAAAAAIBiAAQALAAADADQACAEAAAJIAAAVQA0gvA2AAQAeAAAaAQQAbAQAPAeQAQAfABAoQgBA5gXAmQgYAkglAQQgjAQgoAAQggAAgdgIIAACKQABAWAEAKQAFAKANAEQAOAEAaAAQABAAAAABQABAAAAAAQAAABAAABQABAAAAABQAAABgBABQAAABAAABQAAAAgBAAQAAABgBAAgAg0iyIAACyQAQASASAJQARAJAYAAQBJAAAAh7QAAg3gagdQgageglAAQghAAgaAXg");
	this.shape_7.setTransform(-23.5,3.4,0.413,0.413);

	this.shape_8 = new cjs.Shape();
	this.shape_8.graphics.f("#FFFFFF").s().p("AhJCLQAAAAAAAAQgBAAAAgBQAAAAAAgBQAAgBAAgBQAAgBAAgBQAAgBAAAAQAAgBABAAQAAAAAAAAQARAAAJgEQAIgDADgLQAEgLAAgVIAAifQAAgYgFgLQgEgJgLAAQgVAAgUAWQgUAXgIAhQgBAAAAAAQAAABgBAAQAAAAgBAAQgBAAgBAAQgBAAgBAAQgBAAAAAAQgBAAAAgBQAAAAAAAAIgDhYIABgFQAAgBAFAAID/AAQAFAAAAABIABAFQgDBDAAAVQAAAAgBAAQAAABAAAAQgBAAgBAAQgBAAgBAAQAAAAgBAAQgBAAAAAAQgBAAAAgBQgBAAAAAAQgJghgTgXQgVgWgVAAQgLAAgEAJQgEALAAAYIAACfQAAAVADALQADALAJADQAIAEAQAAQABAAAAAAQABAAAAABQAAAAAAABQABABAAABQAAAEgDAAg");
	this.shape_8.setTransform(-36.9,-0.5,0.413,0.413);

	this.shape_9 = new cjs.Shape();
	this.shape_9.graphics.f("#FFFFFF").s().p("AhAB+QghgUgQgiQgQggAAgkQAAgtAZghQAZgjAngSQAngSApAAQAiAAAcAOQAcAOAAAaQAAANgLAKQgKAKgTAAQgRgBgKgIQgLgLgGgRQgHgVgFgHQgHgIgPABQgeAAgTAeQgSAgAAAxQAAA8AbAjQAaAjA6ABQATAAAOgFQASgGASgJIAAAAQABAAAAABQABAAAAAAQABAAAAABQABAAAAABQABAAAAABQAAAAAAABQAAAAAAABQAAAAAAAAQgcAZgaALQgaAMgjAAQguAAghgUg");
	this.shape_9.setTransform(-49.5,-0.5,0.413,0.413);

	this.shape_10 = new cjs.Shape();
	this.shape_10.graphics.f("#FFFFFF").s().p("AhTB/QgggTgTgkQgSgiAAgoQAAgvAYghQAYghAkgQQAlgQAmAAQAsgBAiAWQAgAUASAjQASAjAAAkQAAArgWAjQgWAggjATQglAUgpAAQguAAghgWgAhLgdQAAAtANAnQAMAlAVAVQAUAUAZAAQAcAAAPgbQAPgcAAg0QAAgygNgkQgOgkgUgSQgWgTgWAAQg6AAAABog");
	this.shape_10.setTransform(-63.1,-0.5,0.413,0.413);

	this.shape_11 = new cjs.Shape();
	this.shape_11.graphics.f("#FFFFFF").s().p("AA4DhQgCAAAAgFQAAAAAAgBQAAgBAAAAQABgBAAAAQABAAAAAAQAXgBANgEQALgDAFgKQAEgLAAgWIAAlDQAAgWgCgLQgCgHgHgDQgGgCgTAAIiTAAQgVAAgIACQgHADgCAHQgCAHAAAaIAAFDQAAAXAEAKQAEALALACQAMAEAZABQAAAAABAAQAAAAAAABQAAAAABABQAAABAAAAQAAACAAAAQgBABAAABQAAAAAAABQgBAAAAAAIjAAAQgBAAAAAAQAAgBAAAAQgBgBAAgBQAAAAAAgCQAAAAAAgBQAAgBABAAQAAgBAAAAQAAAAABAAQAYgBAMgEQAMgDAEgKQAEgLAAgWIAAlDQAAgggKgNQgMgOghAAQgBAAAAAAQAAAAgBgBQAAAAAAgBQAAgBAAgBQAAgBAAgBQAAgBAAAAQABgBAAAAQAAAAABAAIAwABQBgABBpAAQBtAABwgCIAYAAQAAAAABAAQAAAAAAABQABAAAAABQAAABAAABQAAABAAABQAAABgBAAQAAABAAAAQgBAAAAAAQgiAAgLAOQgMANAAAgIAAFDQAAAWAFALQAFAKALADQAMAEAYABQAAAAABAAQAAAAAAABQABAAAAABQAAABAAAAQAAACAAAAQAAABgBABQAAAAAAABQgBAAAAAAg");
	this.shape_11.setTransform(-81.5,-4.1,0.413,0.413);

	this.timeline.addTween(cjs.Tween.get({}).to({state:[{t:this.shape_11},{t:this.shape_10},{t:this.shape_9},{t:this.shape_8},{t:this.shape_7},{t:this.shape_6},{t:this.shape_5},{t:this.shape_4},{t:this.shape_3},{t:this.shape_2},{t:this.shape_1},{t:this.shape}]}).wait(1));

}).prototype = getMCSymbolPrototype(lib.Symbol25, new cjs.Rectangle(-91.8,-13.3,183.7,26.8), null);


(lib.Symbol22 = function(mode,startPosition,loop) {
	this.initialize(mode,startPosition,loop,{});

	// Layer 1
	this.instance = new lib.nammin();
	this.instance.parent = this;
	this.instance.setTransform(-47,-72.5);

	this.timeline.addTween(cjs.Tween.get(this.instance).wait(1));

}).prototype = getMCSymbolPrototype(lib.Symbol22, new cjs.Rectangle(-47,-72.5,94,145), null);


(lib.Symbol19 = function(mode,startPosition,loop) {
	this.initialize(mode,startPosition,loop,{});

	// Layer 1
	this.shape = new cjs.Shape();
	this.shape.graphics.f("#FFFFFF").s().p("A5iZjMAAAgzFMAzFAAAMAAAAzFg");
	this.shape.setTransform(0,0,0.734,1.223);

	this.timeline.addTween(cjs.Tween.get(this.shape).wait(1));

}).prototype = p = new cjs.MovieClip();
p.nominalBounds = new cjs.Rectangle(-120,-200,240,400);


(lib.Symbol6 = function(mode,startPosition,loop) {
	this.initialize(mode,startPosition,loop,{});

	// Layer 1
	this.instance = new lib.btmin();
	this.instance.parent = this;
	this.instance.setTransform(-128.5,-39.5);

	this.timeline.addTween(cjs.Tween.get(this.instance).wait(1));

}).prototype = getMCSymbolPrototype(lib.Symbol6, new cjs.Rectangle(-128.5,-39.5,257,79), null);


(lib.Symbol24 = function(mode,startPosition,loop) {
	this.initialize(mode,startPosition,loop,{});

	// timeline functions:
	this.frame_0 = function() {
		this.stop();
	}
	this.frame_37 = function() {
		this.gotoAndPlay(1);
	}

	// actions tween:
	this.timeline.addTween(cjs.Tween.get(this).call(this.frame_0).wait(37).call(this.frame_37).wait(1));

	// Layer 1
	this.instance = new lib.Symbol22();
	this.instance.parent = this;
	this.instance.setTransform(0,0.1,0.618,0.618,-60);

	this.timeline.addTween(cjs.Tween.get(this.instance).to({regX:0.1,rotation:0,x:0.1,y:0},29).to({regX:0,rotation:-60,x:0,y:0.1},8).wait(1));

}).prototype = p = new cjs.MovieClip();
p.nominalBounds = new cjs.Rectangle(-53.3,-47.5,106.7,95.1);


(lib.Symbol10copy = function(mode,startPosition,loop) {
	this.initialize(mode,startPosition,loop,{});

	// Layer 8
	this.instance = new lib.Symbol26();
	this.instance.parent = this;
	this.instance.compositeOperation = "lighter";

	this.timeline.addTween(cjs.Tween.get(this.instance).wait(1));

}).prototype = getMCSymbolPrototype(lib.Symbol10copy, new cjs.Rectangle(-87.7,-291.5,175.5,583), null);


(lib.Symbol8 = function(mode,startPosition,loop) {
	this.initialize(mode,startPosition,loop,{});

	// timeline functions:
	this.frame_0 = function() {
		this.stop();
	}
	this.frame_59 = function() {
		this.gotoAndPlay(1);
	}

	// actions tween:
	this.timeline.addTween(cjs.Tween.get(this).call(this.frame_0).wait(59).call(this.frame_59).wait(1));

	// Layer 4 (mask)
	var mask = new cjs.Shape();
	mask._off = true;
	mask.graphics.p("AksCGQAAAAAAAAQAAgBgBAAQAAAAAAAAQAAgBAAAAIAAAAIABgCIAAAAQAHAAAEgBIAAAAQADgCACgEIAAAAQABgFAAgIIAAAAIAAiUQAAgMgFgEIAAAAQgEgEgLAAIAAAAIAAgCIAAAAIAAgCIAAAAIApAAQAEAAABACIAAAAQABABAAAEIAAAAIAAAJQAWgUAWAAIAAAAQANAAALAHIAAAAQAKAGAHANIAAAAQAHANAAAQIAAAAQAAAXgKAPIAAAAQgKAQgPAGIAAAAQgPAHgQAAIAAAAQgOAAgMgDIAAAAIAAA5QAAAIACAFIAAAAQACAEAGACIAAAAQAGABALAAIAAAAIABACIAAAAQAAAAAAABQAAAAgBAAQAAAAAAABQAAAAAAAAIAAAAgAkAgnIAABJQAHAHAHAEIAAAAQAHADALAAIAAAAQAeAAAAgyIAAAAQAAgXgLgMIAAAAQgLgMgPAAIAAAAQgOAAgLAKgAKcAvQgNgIgIgOIAAAAQgHgPAAgQIAAAAQAAgTAKgOIAAAAQAKgNAPgHIAAAAQAPgHAQAAIAAAAQASAAAOAJIAAAAQAOAJAHAOIAAAAQAHAOAAAQIAAAAQAAARgJAOIAAAAQgJAOgPAHIAAAAQgPAIgRAAIAAAAQgTAAgOgJgAKggRQAAATAFAPIAAAAQAFAQAJAIIAAAAQAIAJALAAIAAAAQALAAAGgMIAAAAQAGgLAAgWIAAAAQAAgTgFgPIAAAAQgFgPgJgIIAAAAQgJgHgJAAIAAAAQgYAAAAAqgAh1AvQgOgIgHgOIAAAAQgIgPAAgQIAAAAQAAgTAKgOIAAAAQAKgNAPgHIAAAAQAPgHAQAAIAAAAQATAAANAJIAAAAQAOAJAHAOIAAAAQAIAOAAAQIAAAAQAAAQgJAPIAAAAQgJANgPAIIAAAAQgPAIgSAAIAAAAQgSAAgOgJgAhygRQAAATAGAPIAAAAQAFAQAIAIIAAAAQAJAJAKAAIAAAAQAMAAAGgMIAAAAQAGgLAAgWIAAAAQAAgUgFgOIAAAAQgGgPgIgIIAAAAQgJgHgKAAIAAAAQgYAAAAAqgAqZAvQgNgIgIgOIAAAAQgHgPAAgQIAAAAQAAgTAJgOIAAAAQALgNAOgHIAAAAQAQgHAQAAIAAAAQASAAAOAJIAAAAQANAJAIAOIAAAAQAHAOAAAQIAAAAQAAAQgJAPIAAAAQgJANgPAIIAAAAQgPAIgRAAIAAAAQgTAAgOgJgAqVgRQAAASAFAQIAAAAQAFAQAJAIIAAAAQAIAJAKAAIAAAAQAMAAAGgMIAAAAQAGgLAAgWIAAAAQAAgUgFgOIAAAAQgGgPgIgIIAAAAQgJgHgJAAIAAAAQgYAAAAAqgADsAxIgDgCIAAAAIgBgDIAAAAIgCgdQAAAAAAAAQAAAAABAAQAAgBAAAAQAAAAABAAIAAAAIACABIAAAAQAGAQAKAJIAAAAQAKAJAOAAIAAAAQAJAAAFgHIAAAAQAFgHAAgMIAAAAQAAgQgIgHIAAAAQgHgIgSAAIAAAAQAAAAgBAAQAAAAAAAAQAAAAgBgBQAAAAAAgBIAAAAQAAAAAAAAQAAgBABAAQAAAAAAAAQAAAAABAAIAAAAQAQAAAHgGIAAAAQAGgGAAgNIAAAAQAAgKgEgHIAAAAQgFgGgIAAIAAAAQgJAAgJAIIAAAAQgIAIgIAQIAAAAIgCAAIAAAAQgBAAAAAAQAAAAgBgBQAAAAAAAAQAAAAAAgBIAAAAIAGgnIACgBIAAAAIABABIABABIAAAAIAAADQAAAFAFAAIAAAAIAIgDIAAAAQAMgEANAAIAAAAQASAAAKAHIAAAAQAKAGAAAMIAAAAQAAAKgJAIIAAAAQgJAIgQAEIAAAAQATABAKAJIAAAAQALAIAAAPIAAAAQAAAQgPAJIAAAAQgPAJgXAAIAAAAQgYAAgNgHgAoJAvQgOgIgGgOIAAAAQgHgNAAgOIAAAAQAAgTAKgOIAAAAQALgOAQgIIAAAAQAQgHARAAIAAAAQAOAAAMAGIAAAAQALAGAAAKIAAAAQAAAGgEAEIAAAAQgEADgIAAIAAAAQgHAAgEgDIAAAAQgFgEgCgIIAAAAQgDgIgCgDIAAAAQgDgDgGAAIAAAAQgNAAgIAMIAAAAQgHANAAAVIAAAAQAAAYALAOIAAAAQALAPAYAAIAAAAQAIAAAGgCIAAAAIAOgGIAAAAIAAAAIACABIAAAAQAAAAABABQAAAAAAAAQAAAAAAAAQAAABgBAAIAAAAQgLAKgLAFIAAAAQgKAFgPAAIAAAAQgTAAgOgJgAGiAyQgFgFgBgHIAAAAIgOAHIgQAHIAAAAQgIACgHAAIAAAAQgJAAgGgFIAAAAQgFgFAAgJIAAAAQAAgHADgEIAAAAQADgEAHgDIAAAAIAUgIIAAAAIAfgJIAAgXQAAgRgEgJIAAAAQgEgJgIAAIAAAAQgHAAgEAGIAAAAQgEAFgBAMIAAAAQgCARgQAAIAAAAQgOAAAAgMIAAAAQAAgIAKgIIAAAAQALgIAOgFIAAAAQAOgFALAAIAAAAQAOAAAJAIIAAAAQAIAIAAAQIAAAAIAAA0QAAAJADAEIAAAAQAEAFAFAAIAAAAQAFAAAEgCIAAAAIAAAAIACABIAAAAQAAABAAAAQAAAAAAAAQAAABAAAAQgBAAAAAAIAAAAIgeAPIgDAAIAAAAQgFAAgEgEgAGKALQgIACgDAEIAAAAQgEAEAAAFIAAAAQAAAGAEAEIAAAAQADADAGAAIAAAAQAGAAAJgEIAAAAIAEgCIAAgcgANyA0IgFgBIAAAAIgHgIIAAAAIgMgTIAAAAIgMgTIAAAAQgFgFgEgCIAAAAQgFgCgGAAIAAAAIgBAAIAAAgQAAAJACAEIAAAAQABAFADABIAAAAQAEABAHAAIAAAAIAAACIAAAAQAAABAAAAQAAAAAAABQAAAAAAAAQAAAAAAAAIAAAAIg8AAQgBAAAAAAQAAAAAAAAQAAgBAAAAQAAAAAAgBIAAAAIABgCIAAAAQAGAAAEgBIAAAAQAEgBABgFIAAAAQABgDAAgKIAAAAIAAhCQAAgJgBgEIAAAAQgBgEgEgCIAAAAQgDgBgHAAIAAAAIgBgCIAAAAIABgCIAAAAIA/AAIABACIAAAAIgBACIAAAAQgIAAgFABIAAAAQgEACgBAEIAAAAQgCAFAAAJIAAAAIAAAcIAPgBQAGAAAHgQIAAAAQAMgXAIgIIAAAAQAIgIAKAAIAAAAQAJAAAEAEIAAAAQAEAEAAAHIAAAAQAAAHgDAEIAAAAQgEAEgIAAIAAAAQgEAAgFgCIAAAAQgEgDgFAAIAAAAQgEAAgDADIAAAAQgEAEgEAKIAAAAQgFAJgFAEIAAAAQAJABAFADIAAAAQAGADAEAEIAAAAIAMARIAAAAIAHAKIAAAAQAGALAFAFIAAAAQAFAEAJAAIAAAAIABACIAAAAQAAABgBAAQAAAAAAABQAAAAAAAAQAAAAAAAAIAAAAgAJCA0IgLAAIAAAAIAAgCIAAAAIAAgCIAAAAQAHAAADgCIAAAAQADgDAAgJIAAAAIgGhKIgiBVIgCABIAAAAIgCgBIAAAAIgqhTIgDA7QgBANAFAHIAAAAQAFAHAKAAIAAAAIABACIAAAAQAAABAAAAQgBAAAAABQAAAAAAAAQAAAAAAAAIAAAAIgKAAIAAAAIgYAAIgLAAIAAAAQAAAAAAAAQAAAAAAAAQAAgBAAAAQAAAAAAgBIAAAAIAAgCIAAAAQASAAACgbIAAAAIAEhEQgFgHgDgCIAAAAQgFgCgFAAIAAAAIgBgCIAAAAIABgCIAAAAIAeAAQAHAAAEADIAAAAQAEADAFAJIAAAAIAbA4IAag/QABgEADgCIAAAAQACgCADAAIAAAAIAkAAIABACIAAAAIgBACIAAAAQgJAAgFAEIAAAAQgEAEABALIAAAAIAFBDIACANIAAAAQABAFAEABIAAAAQADABAHAAIAAAAIABACIAAAAQAAABAAAAQAAAAAAABQgBAAAAAAQAAAAAAAAIAAAAgABPA0QgBAAAAAAQAAAAAAAAQAAgBAAAAQAAAAAAgBIAAAAIABgCIAAAAQAGAAAEgBIAAAAQAEgBABgFIAAAAQABgDAAgKIAAAAIAAg1Ig0AzIAAACQAAAJABAEIAAAAQABAFADABIAAAAQAEABAHAAIAAAAIAAACIAAAAIAAACIAAAAIg7AAQgBAAAAAAQAAAAAAAAQAAgBAAAAQAAAAAAgBIAAAAIABgCIAAAAQAFAAAEgBIAAAAQAEgBABgFIAAAAQABgEAAgJIAAAAIAAhCQAAgJgBgEIAAAAQgBgEgEgCIAAAAQgEgBgFAAIAAAAIgBgCIAAAAQAAAAAAgBQAAAAAAAAQAAAAABgBQAAAAAAAAIAAAAIA7AAIAAACIAAAAIAAACIAAAAQgHAAgEACIAAAAQgDABgBAFIAAAAQgBAEAAAJIAAAAIAAA0IA0gzIAAgCQAAgJgBgEIAAAAQgBgEgEgCIAAAAQgEgBgGAAIAAAAQAAAAgBAAQAAAAAAgBQAAAAAAAAQAAAAAAgBIAAAAQAAAAAAgBQAAAAAAAAQAAAAAAgBQABAAAAAAIAAAAIA8AAIAAACIAAAAIAAACIAAAAQgHAAgEACIAAAAQgDABgBAFIAAAAQgCAEAAAJIAAAAIAABBQAAAJACAEIAAAAQABAFADABIAAAAQAEABAHAAIAAAAIAAACIAAAAIAAACIAAAAgAmPA0QAAAAAAAAQAAAAAAAAQAAgBAAAAQAAAAAAgBIAAAAIAAgCIAAAAQAHAAAEgBIAAAAQADgBACgFIAAAAQABgEAAgJIAAAAIAAhBQAAgKgCgEIAAAAQgCgEgEAAIAAAAQgJAAgIAJIAAAAQgIAJgEAOIAAAAIgCAAIAAAAIgBAAIAAAAIgCgkIABgDIACAAIAAAAIBpAAIADAAIAAAAIAAADIgBAkIAAAAIgCAAIAAAAIgCAAIAAAAQgEgOgIgJIAAAAQgIgJgJAAIAAAAQgEAAgCAEIAAAAQgCAEAAAKIAAAAIAABBQAAAJACAEIAAAAQABAFADABIAAAAQAEABAGAAIAAAAQABAAAAABQAAAAAAAAQAAAAAAAAQAAABAAAAIAAAAQAAABAAAAQAAAAAAABQAAAAAAAAQAAAAgBAAIAAAAgAsXA0QgBAAAAAAQAAAAAAAAQAAgBAAAAQAAAAAAgBIAAAAQAAAAAAgBQAAAAAAAAQAAAAAAAAQAAgBABAAIAAAAQAJAAAGgBIAAAAQAEgBACgFIAAAAQACgEAAgJIAAAAIAAiFIgBgNIAAAAQgBgEgDgBIAAAAQgCgBgIAAIAAAAIg9AAQgJAAgDABIAAAAQgDABgBAEIAAAAIgBANIAAAAIAACFQAAAJACAEIAAAAQACAFAEABIAAAAQAFABAKAAIAAAAIABACIAAAAQAAABAAAAQAAAAAAABQAAAAAAAAQAAAAgBAAIAAAAIhPAAIgBgCIAAAAIABgCIAAAAQAKAAAFgBIAAAAQAFgBACgFIAAAAQABgEAAgJIAAAAIAAiFQAAgNgEgGIAAAAQgFgFgOAAIAAAAQAAAAAAAAQAAAAAAgBQAAAAAAAAQAAgBAAAAIAAAAIAAgCIAAAAIAUAAIBUABIAAAAIBbgBIAAAAIAKAAIABACIAAAAQAAAAAAABQgBAAAAAAQAAABAAAAQAAAAAAAAIAAAAQgOAAgFAFIAAAAQgFAGAAANIAAAAIAACFQAAAJACAEIAAAAQACAFAFABIAAAAQAFABAKAAIAAAAIABACIAAAAQAAABAAAAQgBAAAAABQAAAAAAAAQAAAAAAAAIAAAAgAAvhaQgLgFgFgGIAAAAQgHgGAAgHIAAAAQAAgGAEgDIAAAAQADgDAGAAIAAAAQAGAAADADIAAAAQADACABAEIAAAAIACAJQABAIADAFIAAAAQAEAEAIAAIAAAAQAJAAAEgFIAAAAQAEgEABgJIAAAAQACgIADgFIAAAAQACgEAHAAIAAAAQAGAAADADIAAAAQAEADAAAGIAAAAQAAAHgGAHIAAAAQgHAGgKAEIAAAAQgKAEgLAAIAAAAQgMAAgKgEg");
	mask.setTransform(1.2,1.3);

	// Layer 5
	this.instance = new lib.Symbol27();
	this.instance.parent = this;
	this.instance.setTransform(-155.9,-13,1,1.051,0,17.9,0);

	var maskedShapeInstanceList = [this.instance];

	for(var shapedInstanceItr = 0; shapedInstanceItr < maskedShapeInstanceList.length; shapedInstanceItr++) {
		maskedShapeInstanceList[shapedInstanceItr].mask = mask;
	}

	this.timeline.addTween(cjs.Tween.get(this.instance).to({x:163.1},33).wait(27));

	// Layer 3
	this.instance_1 = new lib.Symbol25();
	this.instance_1.parent = this;
	this.instance_1.setTransform(1.2,1.3);

	this.timeline.addTween(cjs.Tween.get(this.instance_1).wait(60));

	// Layer 1
	this.instance_2 = new lib.Symbol6();
	this.instance_2.parent = this;

	this.timeline.addTween(cjs.Tween.get(this.instance_2).wait(60));

}).prototype = p = new cjs.MovieClip();
p.nominalBounds = new cjs.Rectangle(-128.5,-39.5,257,79);


(lib.Symbol15r = function(mode,startPosition,loop) {
	this.initialize(mode,startPosition,loop,{});

	// Layer 1
	this.instance = new lib.Symbol39();
	this.instance.parent = this;
	this.instance.cache(-23,-22,47,44);

	this.timeline.addTween(cjs.Tween.get(this.instance).wait(1));

}).prototype = getMCSymbolPrototype(lib.Symbol15r, new cjs.Rectangle(-21.2,-19.8,42.5,39.8), null);


(lib.Symbol13r = function(mode,startPosition,loop) {
	this.initialize(mode,startPosition,loop,{});

	// Layer 1
	this.instance = new lib.Symbol15r();
	this.instance.parent = this;
	this.instance.filters = [new cjs.BlurFilter(3, 3, 1)];
	this.instance.cache(-23,-22,47,44);

	this.timeline.addTween(cjs.Tween.get(this.instance).wait(1));

}).prototype = getMCSymbolPrototype(lib.Symbol13r, new cjs.Rectangle(-23.2,-21.8,50,46), null);


(lib.Symbol14r = function(mode,startPosition,loop) {
	this.initialize(mode,startPosition,loop,{});

	// Layer 11 copy
	this.instance = new lib.Symbol13r();
	this.instance.parent = this;
	this.instance.setTransform(27.8,-71.9,0.702,0.702);
	this.instance.alpha = 0.691;

	this.timeline.addTween(cjs.Tween.get(this.instance).to({scaleX:1.61,scaleY:1.61,guide:{path:[27.9,-72,34.7,-84.4,42.7,-96.3,79.8,-151,144.1,-196.5]},alpha:0},74).wait(1).to({scaleX:0.29,scaleY:0.29,x:0,y:0,alpha:1},0).to({scaleX:0.7,scaleY:0.7,guide:{path:[0.1,0,1.2,-5,2.6,-10,11.5,-42.1,27.7,-71.8]},alpha:0.691},72).wait(1));

	// Layer 9 copy
	this.instance_1 = new lib.Symbol13r();
	this.instance_1.parent = this;
	this.instance_1.setTransform(17.6,-68.1,0.629,0.629);
	this.instance_1.alpha = 0.691;

	this.timeline.addTween(cjs.Tween.get(this.instance_1).to({scaleX:1.37,scaleY:1.37,guide:{path:[17.7,-68.1,45.1,-165.6,74.5,-214.8]},alpha:0},74).wait(1).to({scaleX:0.29,scaleY:0.29,x:0,y:0,alpha:1},0).to({scaleX:0.63,scaleY:0.63,guide:{path:[0.1,0,8.8,-36.5,17.6,-68.1]},alpha:0.691},72).wait(1));

	// Layer 7 copy
	this.instance_2 = new lib.Symbol13r();
	this.instance_2.parent = this;
	this.instance_2.setTransform(21.9,-59.8,1.361,1.361);
	this.instance_2.alpha = 0.691;

	this.timeline.addTween(cjs.Tween.get(this.instance_2).to({scaleX:3.73,scaleY:3.73,guide:{path:[22,-59.9,58.1,-127.6,133.9,-143.2]},alpha:0},74).wait(1).to({scaleX:0.29,scaleY:0.29,x:0,y:0,alpha:1},0).to({scaleX:1.36,scaleY:1.36,guide:{path:[0.1,0,8.1,-33.7,21.9,-59.8]},alpha:0.691},72).wait(1));

	// Layer 5 copy
	this.instance_3 = new lib.Symbol13r();
	this.instance_3.parent = this;
	this.instance_3.setTransform(4.1,-35.2,1.361,1.361);
	this.instance_3.alpha = 0.691;

	this.timeline.addTween(cjs.Tween.get(this.instance_3).to({scaleX:3.73,scaleY:3.73,guide:{path:[4.2,-35.2,11.4,-78.9,27.7,-110.7]},alpha:0},74).wait(1).to({scaleX:0.29,scaleY:0.29,x:0,y:0,alpha:1},0).to({scaleX:1.36,scaleY:1.36,guide:{path:[0.1,0,1.4,-18.4,4,-35.1]},alpha:0.691},72).wait(1));

	// Layer 1 copy
	this.instance_4 = new lib.Symbol13r();
	this.instance_4.parent = this;
	this.instance_4.setTransform(9.5,-53.8,1.112,1.112);
	this.instance_4.alpha = 0.691;

	this.timeline.addTween(cjs.Tween.get(this.instance_4).to({scaleX:2.93,scaleY:2.93,guide:{path:[9.6,-53.8,28.5,-117.2,81.7,-150.3]},alpha:0},74).wait(1).to({scaleX:0.29,scaleY:0.29,x:0,y:0,alpha:1},0).to({scaleX:1.11,scaleY:1.11,guide:{path:[0.1,0,2.2,-29.1,9.4,-53.8]},alpha:0.691},72).wait(1));

	// Layer 11 copy 2
	this.instance_5 = new lib.Symbol13r();
	this.instance_5.parent = this;
	this.instance_5.setTransform(79,-141,1.154,1.154);
	this.instance_5.alpha = 0.34;

	this.timeline.addTween(cjs.Tween.get(this.instance_5).to({scaleX:1.61,scaleY:1.61,guide:{path:[79,-140.9,107,-170.2,144.1,-196.5]},alpha:0},64).wait(1).to({scaleX:0.29,scaleY:0.29,x:0,y:0,alpha:1},0).to({scaleX:1.15,scaleY:1.15,guide:{path:[0.1,0,18.5,-77.6,78.9,-140.9]},alpha:0.34},82).wait(1));

	// Layer 9 copy 2
	this.instance_6 = new lib.Symbol13r();
	this.instance_6.parent = this;
	this.instance_6.setTransform(41.5,-142.8,1.001,1.001);
	this.instance_6.alpha = 0.34;

	this.timeline.addTween(cjs.Tween.get(this.instance_6).to({scaleX:1.37,scaleY:1.37,guide:{path:[41.5,-142.7,57.7,-186.7,74.5,-214.8]},alpha:0},64).wait(1).to({scaleX:0.29,scaleY:0.29,x:0,y:0,alpha:1},0).to({scaleX:1,scaleY:1,guide:{path:[0.1,0,20.2,-85,41.4,-142.8]},alpha:0.34},82).wait(1));

	// Layer 7 copy 2
	this.instance_7 = new lib.Symbol13r();
	this.instance_7.parent = this;
	this.instance_7.setTransform(67.9,-114.5,2.546,2.546);
	this.instance_7.alpha = 0.34;

	this.timeline.addTween(cjs.Tween.get(this.instance_7).to({scaleX:3.73,scaleY:3.73,guide:{path:[67.9,-114.5,95.9,-135.4,133.9,-143.2]},alpha:0},64).wait(1).to({scaleX:0.29,scaleY:0.29,x:0,y:0,alpha:1},0).to({scaleX:2.55,scaleY:2.55,guide:{path:[0.1,0,18.5,-77.5,67.8,-114.4]},alpha:0.34},82).wait(1));

	// Layer 5 copy 2
	this.instance_8 = new lib.Symbol13r();
	this.instance_8.parent = this;
	this.instance_8.setTransform(12.9,-73.8,2.546,2.546);
	this.instance_8.alpha = 0.34;

	this.timeline.addTween(cjs.Tween.get(this.instance_8).to({scaleX:3.73,scaleY:3.73,guide:{path:[13,-73.7,19,-93.9,27.7,-110.7]},alpha:0},64).wait(1).to({scaleX:0.29,scaleY:0.29,x:0,y:0,alpha:1},0).to({scaleX:2.55,scaleY:2.55,guide:{path:[0.1,0,3.1,-41.2,12.9,-73.8]},alpha:0.34},82).wait(1));

	// Layer 1 copy 2
	this.instance_9 = new lib.Symbol13r();
	this.instance_9.parent = this;
	this.instance_9.setTransform(36.3,-108.7,2.021,2.021);
	this.instance_9.alpha = 0.34;

	this.timeline.addTween(cjs.Tween.get(this.instance_9).to({scaleX:2.93,scaleY:2.93,guide:{path:[36.4,-108.7,54.5,-133.4,81.7,-150.3]},alpha:0},64).wait(1).to({scaleX:0.29,scaleY:0.29,x:0,y:0,alpha:1},0).to({scaleX:2.02,scaleY:2.02,guide:{path:[0.1,0,5,-66,36.3,-108.6]},alpha:0.34},82).wait(1));

	// Layer 11
	this.instance_10 = new lib.Symbol13r();
	this.instance_10.parent = this;
	this.instance_10.setTransform(0,0,0.294,0.294);

	this.timeline.addTween(cjs.Tween.get(this.instance_10).to({scaleX:1.61,scaleY:1.61,guide:{path:[0.1,0,27,-113.6,144,-196.4]},alpha:0},147).wait(1));

	// Layer 9
	this.instance_11 = new lib.Symbol13r();
	this.instance_11.parent = this;
	this.instance_11.setTransform(0,0,0.294,0.294);

	this.timeline.addTween(cjs.Tween.get(this.instance_11).to({scaleX:1.37,scaleY:1.37,guide:{path:[0.1,0,35.6,-149.7,74.5,-214.8]},alpha:0},147).wait(1));

	// Layer 7
	this.instance_12 = new lib.Symbol13r();
	this.instance_12.parent = this;
	this.instance_12.setTransform(0,0,0.294,0.294);

	this.timeline.addTween(cjs.Tween.get(this.instance_12).to({scaleX:3.73,scaleY:3.73,guide:{path:[0.1,0,28.9,-121.5,133.8,-143.2]},alpha:0},147).wait(1));

	// Layer 5
	this.instance_13 = new lib.Symbol13r();
	this.instance_13.parent = this;
	this.instance_13.setTransform(0,0,0.294,0.294);

	this.timeline.addTween(cjs.Tween.get(this.instance_13).to({scaleX:3.73,scaleY:3.73,guide:{path:[0.1,0,5,-66.7,27.7,-110.7]},alpha:0},147).wait(1));

	// Layer 1
	this.instance_14 = new lib.Symbol13r();
	this.instance_14.parent = this;
	this.instance_14.setTransform(0,0,0.294,0.294);

	this.timeline.addTween(cjs.Tween.get(this.instance_14).to({scaleX:2.93,scaleY:2.93,guide:{path:[0.1,0,7.8,-104.4,81.6,-150.3]},alpha:0},147).wait(1));

}).prototype = p = new cjs.MovieClip();
p.nominalBounds = new cjs.Rectangle(-43.2,-167.1,171,177.3);


(lib.Symbol21 = function(mode,startPosition,loop) {
	this.initialize(mode,startPosition,loop,{});

	// Layer 4 copy (mask)
	var mask = new cjs.Shape();
	mask._off = true;
	mask.graphics.p("AiSgkIBhgrIDEBjIhwA8IAAAAIi1h0gAiSglIAAABIAAAAIAAgBg");
	mask.setTransform(-12.2,51.2);

	// Layer 5 copy
	this.instance = new lib.Symbol10copy();
	this.instance.parent = this;
	this.instance.setTransform(-66.3,43.3,0.255,0.255,90);

	var maskedShapeInstanceList = [this.instance];

	for(var shapedInstanceItr = 0; shapedInstanceItr < maskedShapeInstanceList.length; shapedInstanceItr++) {
		maskedShapeInstanceList[shapedInstanceItr].mask = mask;
	}

	this.timeline.addTween(cjs.Tween.get(this.instance).to({x:3.7},144).wait(1));

	// Layer 4 (mask)
	var mask_1 = new cjs.Shape();
	mask_1._off = true;
	mask_1.graphics.p("Ah8A/QgVgLgMgOIAAhMQAMgNAVgMQAxgaBEAAQBEAAAwAaQAwAbAAAkQAAAmgwAZQgwAbhEAAQhEAAgxgbg");
	mask_1.setTransform(-49.2,19.7);

	// Layer 5
	this.instance_1 = new lib.Symbol10copy();
	this.instance_1.parent = this;
	this.instance_1.setTransform(-66.3,20.8,0.255,0.255,90);

	var maskedShapeInstanceList = [this.instance_1];

	for(var shapedInstanceItr = 0; shapedInstanceItr < maskedShapeInstanceList.length; shapedInstanceItr++) {
		maskedShapeInstanceList[shapedInstanceItr].mask = mask_1;
	}

	this.timeline.addTween(cjs.Tween.get(this.instance_1).to({x:3.7},144).wait(1));

	// Layer 3 (mask)
	var mask_2 = new cjs.Shape();
	mask_2._off = true;
	mask_2.graphics.p("AhWMQIhVgrQAAgClzgEIl0gEIAA3qIclAAIAAX8IuOAAIhbAjIAAAAg");
	mask_2.setTransform(-6,-135.4);

	// Layer 2
	this.s = new lib.Symbol14r();
	this.s.parent = this;
	this.s.setTransform(-13.6,-54.1,0.472,0.472,-21.6,0,0,-0.5,0.1);

	var maskedShapeInstanceList = [this.s];

	for(var shapedInstanceItr = 0; shapedInstanceItr < maskedShapeInstanceList.length; shapedInstanceItr++) {
		maskedShapeInstanceList[shapedInstanceItr].mask = mask_2;
	}

	this.timeline.addTween(cjs.Tween.get(this.s).wait(145));

	// Layer 1
	this.instance_2 = new lib.housemin();
	this.instance_2.parent = this;
	this.instance_2.setTransform(-128.5,-100);

	this.timeline.addTween(cjs.Tween.get(this.instance_2).wait(145));

}).prototype = p = new cjs.MovieClip();
p.nominalBounds = new cjs.Rectangle(-128.5,-149.9,257,250);


(lib.Symbol23 = function(mode,startPosition,loop) {
	this.initialize(mode,startPosition,loop,{});

	// timeline functions:
	this.frame_0 = function() {
		this.stop();
	}
	this.frame_14 = function() {
		this.stop();
	}

	// actions tween:
	this.timeline.addTween(cjs.Tween.get(this).call(this.frame_0).wait(14).call(this.frame_14).wait(1));

	// Layer 1
	this.instance = new lib.housemin();
	this.instance.parent = this;
	this.instance.setTransform(-128.5,-76);

	this.instance_1 = new lib.Symbol21();
	this.instance_1.parent = this;
	this.instance_1.setTransform(0,24);
	this.instance_1._off = true;

	this.timeline.addTween(cjs.Tween.get({}).to({state:[{t:this.instance}]}).to({state:[{t:this.instance_1}]},1).to({state:[{t:this.instance_1}]},1).to({state:[{t:this.instance_1}]},1).to({state:[{t:this.instance_1}]},1).to({state:[{t:this.instance_1}]},1).to({state:[{t:this.instance_1}]},1).to({state:[{t:this.instance_1}]},1).to({state:[{t:this.instance_1}]},1).to({state:[{t:this.instance_1}]},1).to({state:[{t:this.instance_1}]},1).to({state:[{t:this.instance_1}]},1).to({state:[{t:this.instance_1}]},1).to({state:[{t:this.instance_1}]},1).to({state:[{t:this.instance_1}]},1).wait(1));
	this.timeline.addTween(cjs.Tween.get(this.instance_1).wait(1).to({_off:false},0).wait(1).to({regY:-49.9,y:-25.9},0).wait(12).to({regY:0,y:24},0).wait(1));

}).prototype = p = new cjs.MovieClip();
p.nominalBounds = new cjs.Rectangle(-128.5,-76,257,200);


// stage content:
(lib.MKT19593_bl2600_METCH_240x400_003 = function(mode,startPosition,loop) {
	this.initialize(mode,startPosition,loop,{});

	// timeline functions:
	this.frame_0 = function() {
		var self = this;
		var isOver = false;
		stage.enableMouseOver(3);
		createjs.Touch.enable(stage);
		
		self.mcButton.cursor = "none";
		
		//self.holder.scaleX = self.holder.scaleY = 1;
		//self.holder.x = self.holder.y = 0;
		
		
		
		
		//scale = 0.7;
		//count = 30;
		//function init() {
		
		//	for (var i = 0; i < count; i++) {
		//		var c = new lib.star();
		//		c.y = Math.random() * stage.canvas.height;
		//		c.x = Math.random() * stage.canvas.width;
		//		c.gotoAndPlay(Math.ceil(Math.random() * (c.totalFrames - 1)));
		//		//c.speed = 6*Math.random();
		//		c.rotation = Math.random() * 360;
		//		c.scaleX = c.scaleY = Math.random() * scale;
		//		c.on("tick", run_snow);
		//		self.holder.addChildAt(c, i);
		//	}
		//};
		
		
		//function run_snow() {
		//	if (isOver) {
		//		if (this.currentFrame == this.totalFrames - 1) {
		//			this.y = stage.canvas.height * Math.random();
		//			this.x = Math.random() * stage.canvas.width;
		//			//this.scaleX = this.scaleY = Math.random() * scale;
		
		
		//		}
		//	} else {
		//		this.stop();
		//	}
		
		
		//}
		
		
		
		self.mcButton.addEventListener("pressup", up);
		function up() {
		
			window.open(clickTag, "_blank");
		
		
		}
		
		
		
		/*stage.addEventListener("tick", fl_StopAll);
		
		
		function fl_StopAll() {
		
			stage.removeEventListener("tick", fl_StopAll);
		}*/
		
		
		self.mcButton.addEventListener("mouseout", fl_MouseOutHandler);
		function fl_MouseOutHandler() {
			isOver = false;
			self.house.gotoAndStop(0);
			self.cur.gotoAndStop(0);
			self.bt.gotoAndStop(0);
		
		
		}
		
		self.mcButton.addEventListener("mouseover", fl_MouseOverHandler);
		function fl_MouseOverHandler() {
			isOver = true;
		
			self.house.play();
			self.bt.play();
			self.cur.play();
		
		
		}
		
		
		this.addEventListener("tick", cur_play);
		function cur_play() {
			if (isOver) {
		
				self.cur.x = stage.mouseX;
				self.cur.y = stage.mouseY;
		
		
			}
		}
	}

	// actions tween:
	this.timeline.addTween(cjs.Tween.get(this).call(this.frame_0).wait(1));

	// mcButton
	this.mcButton = new lib.Symbol19();
	this.mcButton.parent = this;
	this.mcButton.setTransform(120,200);
	this.mcButton.alpha = 0.012;
	new cjs.ButtonHelper(this.mcButton, 0, 1, 1);

	this.timeline.addTween(cjs.Tween.get(this.mcButton).wait(1));

	// cur
	this.cur = new lib.Symbol24();
	this.cur.parent = this;
	this.cur.setTransform(320.6,146.5);

	this.timeline.addTween(cjs.Tween.get(this.cur).wait(1));

	// bt
	this.bt = new lib.Symbol8();
	this.bt.parent = this;
	this.bt.setTransform(120.1,360.7,0.681,0.681,0,0,0,0.1,0.1);

	this.timeline.addTween(cjs.Tween.get(this.bt).wait(1));

	// 18
	this.shape = new cjs.Shape();
	this.shape.graphics.f("#FFFFFF").s().p("AgoE9IACg0IAAjgIjvACIAAhXIA1ACIC6AAIAAjcIgCg2IBSAAIgCESIC1AAIA4gCIAABXQgTgCghAAIi5AAIACEUg");
	this.shape.setTransform(234.1,393.2,0.177,0.177);

	this.shape_1 = new cjs.Shape();
	this.shape_1.graphics.f("#FFFFFF").s().p("AiXGIQgugKgigVQgjgUgXgiQgWghAAgvQAAgxAOgkQANgjAbgeQAYgaAngaQAigXAxgWQgZgLgdgQQgZgPgRgRQgQgRgMgaQgKgYAAgiQAAgwASgkQASgjAegYQAegZAjgPQAlgOAjgKQAlgIAdgDQAfgDATAAQAgAAAfAIQAeAGAZARQAXARAPAZQAPAaAAAjQAAAkgTAfQgRAegdAZQgeAbggASQgiATgfAOIBUAdQAxARAkAVQAoAZAYAhQAaAiAAA1QgCAwgUAmQgUAmggAdQggAdgpATQglATgsANQgoAMgoAHQgnAFgaAAQgsAAgugKgAh2AYQgSATgPAWQgOAXgIAaQgJAcAAAbQAAAxAUAjQAUAjAfAWQAgAWAlALQAlAKAiAAQBAAAAogmQAognABhPQAAgtgagiQgagggngZQgmgXgsgQIhOgaQgXALgSARgAhTlGQgZAjAAAzQAAAoAOAbQAOAbASARQASARAWAKIAkARQATgHAOgHQARgIAPgOQAQgQAJgUQAKgVAAgeQAAhFghgpQgggrg8AAQguAAgaAjg");
	this.shape_1.setTransform(222.5,391.9,0.177,0.177);

	this.shape_2 = new cjs.Shape();
	this.shape_2.graphics.f("#FFFFFF").s().p("AinGIQABgHAEgDQAEgCAFgBIApgIQAVgFACgIQAEgTAGg0QAFgsAEhQQAFhBAChcQADhGgBhkQABg5gDgeQgDgmgGgRIgrASQgaAKgVALQgEAAgGgIQgGgIAAgIIACgHIA9gTQAlgMApgOIBMgcQAngQAUgOQAAABAAAAQAAABAAAAQABABAAAAQABAAAAAAQACABAAAEQgHAMgFAeQgHAigDApQgFAtgDA0IgFBoIgED5IADBCQACAcADAcQAEAaAGAOQADAJAJAGQAKAIANAEQALAGAQADQAPADALAAQAFAAACAEQACAEABADQAAALgJAAQgcACgegBIiVgCQgjAAgiABIgxAFQgMAAAAgKg");
	this.shape_2.setTransform(212.8,391.8,0.177,0.177);

	this.timeline.addTween(cjs.Tween.get({}).to({state:[{t:this.shape_2},{t:this.shape_1},{t:this.shape}]}).wait(1));

	// logo
	this.instance = new lib.logomin();
	this.instance.parent = this;
	this.instance.setTransform(18,10,1.02,1.02);

	this.timeline.addTween(cjs.Tween.get(this.instance).wait(1));

	// house
	this.house = new lib.Symbol23();
	this.house.parent = this;
	this.house.setTransform(113.3,144.1,1,1,0,0,0,-6.2,-33);

	this.timeline.addTween(cjs.Tween.get(this.house).wait(1));

	// Layer 1
	this.shape_3 = new cjs.Shape();
	this.shape_3.graphics.lf(["rgba(66,66,66,0)","#000000"],[0,1],9,-32.4,9,-162.7).s().p("A3qXrMAAAgvVMAvVAAAMAAAAvVg");
	this.shape_3.setTransform(120,200,0.792,1.32);

	this.timeline.addTween(cjs.Tween.get(this.shape_3).wait(1));

	// back
	this.instance_1 = new lib.back();
	this.instance_1.parent = this;
	this.instance_1.setTransform(-34,0);

	this.timeline.addTween(cjs.Tween.get(this.instance_1).wait(1));

}).prototype = p = new cjs.MovieClip();
p.nominalBounds = new cjs.Rectangle(73.5,200,420.4,420);
// library properties:
lib.properties = {
	width: 240,
	height: 400,
	fps: 40,
	color: "#152C3A",
	opacity: 1.00,
	webfonts: {},
	manifest: [
		{src:"images/back.jpg", id:"back"},
		{src:"images/btmin.png", id:"btmin"},
		{src:"images/housemin.png", id:"housemin"},
		{src:"images/logomin.png", id:"logomin"},
		{src:"images/mg.jpg", id:"mg"},
		{src:"images/nammin.png", id:"nammin"}
	],
	preloads: []
};




})(lib = lib||{}, images = images||{}, createjs = createjs||{}, ss = ss||{}, AdobeAn = AdobeAn||{});
var lib, images, createjs, ss, AdobeAn;