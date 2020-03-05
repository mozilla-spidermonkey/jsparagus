(function(global, createjs){
	(function (cjs, an) {

var p; // shortcut to reference prototypes
var lib={};var ss={};var img={};
lib.ssMetadata = [];


// symbols:
// helper functions:

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


(lib.Shape_Drop_Shadow = function(mode,startPosition,loop) {
	this.initialize(mode,startPosition,loop,{});

	// Layer 1
	this.shape = new cjs.Shape();
	this.shape.graphics.f("#000000").s().p("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");
	this.shape.setTransform(0.0056,-0.003);

	this.timeline.addTween(cjs.Tween.get(this.shape).wait(1));

}).prototype = getMCSymbolPrototype(lib.Shape_Drop_Shadow, new cjs.Rectangle(-65.7,-90,131.5,180), null);


(lib.CreditValueContainer = function(mode,startPosition,loop) {
	this.initialize(mode,startPosition,loop,{});

	// Layer 1
	this.creditLangField_txt = new cjs.Text("CREDITS", "bold 12px 'Arial'", "#FFCC00");
	this.creditLangField_txt.name = "creditLangField_txt";
	this.creditLangField_txt.textAlign = "center";
	this.creditLangField_txt.lineHeight = 13;
	this.creditLangField_txt.lineWidth = 105;
	this.creditLangField_txt.parent = this;
	this.creditLangField_txt.setTransform(0,2.25);
	this.creditLangField_txt.shadow = new cjs.Shadow("rgba(0,0,0,1)",0,0,10);

	this.creditValueField_txt = new cjs.Text("9999", "bold 32px 'Arial'", "#FFCC00");
	this.creditValueField_txt.name = "creditValueField_txt";
	this.creditValueField_txt.textAlign = "center";
	this.creditValueField_txt.lineHeight = 36;
	this.creditValueField_txt.lineWidth = 105;
	this.creditValueField_txt.parent = this;
	this.creditValueField_txt.setTransform(0,-33.3);
	this.creditValueField_txt.shadow = new cjs.Shadow("rgba(0,0,0,1)",0,0,10);

	this.timeline.addTween(cjs.Tween.get({}).to({state:[{t:this.creditValueField_txt},{t:this.creditLangField_txt}]}).wait(1));

}).prototype = getMCSymbolPrototype(lib.CreditValueContainer, new cjs.Rectangle(-96.3,-77.3,199,130), null);


(lib.Surprise = function(mode,startPosition,loop) {
	this.initialize(mode,startPosition,loop,{});

	// Surprise
	this.shape = new cjs.Shape();
	this.shape.graphics.f("#FCEEDC").s().p("AAdAtQgXgFgUgKQgggigQgqQAqApAwABIATABQAPAkABAOg");
	this.shape.setTransform(40.85,62.85);

	this.shape_1 = new cjs.Shape();
	this.shape_1.graphics.f("#FCEEDC").s().p("Ag5gHQgIgYgCgMQA1ArBHAFQAJAZABAOQhFgDg3gwg");
	this.shape_1.setTransform(34.45,50.3);

	this.shape_2 = new cjs.Shape();
	this.shape_2.graphics.f("#E4BE9D").s().p("AgTAJQAUgPAFgTIAAgQQARAQgDAWQgCAKgFAIQgHANgNAOIgMghg");
	this.shape_2.setTransform(52.9481,47.025);

	this.shape_3 = new cjs.Shape();
	this.shape_3.graphics.f("#FCEEDC").s().p("AgwAMQgMgagCgYIADACQAZAgAiAFQAaAEAcgIQAIAYABAIQgXAKgWAAQgiAAgggbg");
	this.shape_3.setTransform(33.4,23.4252);

	this.shape_4 = new cjs.Shape();
	this.shape_4.graphics.f("#FCE2C4").s().p("AgLAIQAOgMgGgSIgKgNQATAOADAKQAOAYgXAYQgCgMgJgRg");
	this.shape_4.setTransform(42.4266,21.1);

	this.shape_5 = new cjs.Shape();
	this.shape_5.graphics.f("#FCEEDC").s().p("AAXAlIgJgCQgJAAgQgFQgfgcgFgpQALAQARAIIAMAFQATAGAEAAIAOACQAFAQANAaQgOgBgLgCg");
	this.shape_5.setTransform(10.625,-8.05);

	this.shape_6 = new cjs.Shape();
	this.shape_6.graphics.f("#E4BE9D").s().p("AABAuQgNgFgIgOQgRgdgBgMQgBgTAbgKIAGgCQARgGAJAIQAHAGAFAQIAHAgQADAVgLAJQgJAHgKAAQgFAAgGgCgAgKgoIgFACQgXAJABAQQABAKAPAaQAGANAMAEQANAFALgKQAJgIgCgSQgBgFgGgWQgEgXgRAAIgKABg");
	this.shape_6.setTransform(26.6464,9.0083);

	this.shape_7 = new cjs.Shape();
	this.shape_7.graphics.f("#FCE2C4").s().p("AABAuQgOgFgHgOQgSgdAAgMQgCgTAcgKIAGgCQAfgKAHAiIAHAgQADAVgLAJQgJAHgLAAQgFAAgFgCg");
	this.shape_7.setTransform(26.5443,8.903);

	this.shape_8 = new cjs.Shape();
	this.shape_8.graphics.f("#A57258").s().p("AgKgDIhOgWIgGgHIABgBQCfArAWAMQAKAHgEAFQgXgNhRgYg");
	this.shape_8.setTransform(38.7204,13.125);

	this.shape_9 = new cjs.Shape();
	this.shape_9.graphics.f("#CC9579").s().p("AAwAcQgdgIg9gXIg4gVIAJgJQCeAqAWAMQALAIgEAEIgNABQgQAAgVgGg");
	this.shape_9.setTransform(38.3188,13.1365);

	this.shape_10 = new cjs.Shape();
	this.shape_10.graphics.f("#E4BE9D").s().p("AAoApQgdgJg9gXIg4gVIACgjIBSAAQBYAFAgAYIAIARQAFAVgPAYQgCACgJABIgFABQgQAAgYgHg");
	this.shape_10.setTransform(39.1641,11.8618);

	this.shape_11 = new cjs.Shape();
	this.shape_11.graphics.f("#A57258").s().p("AhnBhQAvgdArgzQAzg/AUgXQArgtADAVQgBgDgFADIgRAPQgNALgkArQgtA3gUAVQgWATgMAJQgTAOgRAGg");
	this.shape_11.setTransform(41.05,-1.1841);

	this.shape_12 = new cjs.Shape();
	this.shape_12.graphics.f("#CC9579").s().p("AA4guQhEBBgrAcQARgfBeg+g");
	this.shape_12.setTransform(16.95,14.85);

	this.shape_13 = new cjs.Shape();
	this.shape_13.graphics.f("#CC9579").s().p("AhNgdQAMgEAcAJQArAOBIApQh5gqgigSg");
	this.shape_13.setTransform(9.1,0.976);

	this.shape_14 = new cjs.Shape();
	this.shape_14.graphics.f("#002A00").s().p("AAAgGIAKAGQgJAEgKADQADgGAGgHg");
	this.shape_14.setTransform(24.6,-4.95);

	this.shape_15 = new cjs.Shape();
	this.shape_15.graphics.f("#FCE2C4").s().p("AhoA5QgmgmgRg5QgPg0AJgfQABAAAAAAQABAAAAAAQAAAAAAAAQAAAAAAAAQgJB4BmA3QAoAWAuAFQAsAFAegMQAxgTANgdQAHgOgDgLQAQAQgDAWQgBALgFAJQgYA1hRAJIgVABQhNAAhAhBg");
	this.shape_15.setTransform(37.9293,42.7099);

	this.shape_16 = new cjs.Shape();
	this.shape_16.graphics.f("#FCE2C4").s().p("AAIBUIgugDQhTgSgfg/QgRgjAAgwQABAkAnAmQAqAnAwABQAnAGBYgRQAtgJAlgJQgYAThMAgIA9ATQg9AMg5AAIgFAAg");
	this.shape_16.setTransform(47.6749,59.326);

	this.shape_17 = new cjs.Shape();
	this.shape_17.graphics.f("#FCE2C4").s().p("AgFAyIgigEQgsgEgSgjQgPgcAIgcQAEAcAfAQIAAACIAMADQAMAEAMABQA6ALAzgMQAagFAOgIQgKAOgyAXIAfANQgpAJgmAAIgJAAg");
	this.shape_17.setTransform(16.0328,-9.0906);

	this.shape_18 = new cjs.Shape();
	this.shape_18.graphics.f("#E4BE9D").s().p("AhOA7QgCgUBHgwQAngbArgZIACAAIAEAEQh/B3gZAAQgEAAgBgDg");
	this.shape_18.setTransform(16.2473,14.383);

	this.shape_19 = new cjs.Shape();
	this.shape_19.graphics.f("#FCEEDC").s().p("AgZAKIgYgUQADgMAGgJQANgTATAPQAtAjAMAUQgGAIgRAOIgCAAQgWgKgbgWg");
	this.shape_19.setTransform(14.75,20.2642);

	this.shape_20 = new cjs.Shape();
	this.shape_20.graphics.f("#FCE2C4").s().p("AgoBOQgPgIgdgbQgTgRBTg4QAogcAtgaIAXAVQAGAegkAxQgdApgcAVQgIAHgLAAQgKAAgMgHg");
	this.shape_20.setTransform(17.2234,16.6689);

	this.shape_21 = new cjs.Shape();
	this.shape_21.graphics.f("#E4BE9D").s().p("AgKAAIgGgPIAQgRQAFARAMASQgJAOAAAPQgJgPgJgRg");
	this.shape_21.setTransform(24.85,17.75);

	this.shape_22 = new cjs.Shape();
	this.shape_22.graphics.f("#FCE2C4").s().p("Ag6AbQgTgUgPgcIgLgYIASgSQAaBLA4ANQAmAJAjgUQAYgOgHgSIgLgQQAUAPADAJQALAVgNATQgHAKgJAFQghATgeAAQgpAAgjglg");
	this.shape_22.setTransform(33.5346,20.8284);

	this.shape_23 = new cjs.Shape();
	this.shape_23.graphics.f("#E4BE9D").s().p("AhtgaQgUgNAFgMIAIgKIABAAQAVgIAnAYQBZA6BcAzQjLhEgggWg");
	this.shape_23.setTransform(11.6087,0.1857);

	this.shape_24 = new cjs.Shape();
	this.shape_24.graphics.f("#FCEEDC").s().p("AgKAxQgIgBgJgFQgJgfAIgjIAJgeIAXgEQAZAAACAbQAEAmgKAyQgZgFgKgEg");
	this.shape_24.setTransform(1.8564,1.675);

	this.shape_25 = new cjs.Shape();
	this.shape_25.graphics.f("#FCE2C4").s().p("AhfAzQgNgCgMgJQgXgUAJglQAAgZAMgRQAXgkA4AlQBZA6BcAzIgCAaQimgEhBgWg");
	this.shape_25.setTransform(10.4353,1.4559);

	this.shape_26 = new cjs.Shape();
	this.shape_26.graphics.f("#CC9579").s().p("AhMBQQAngdAcgjQA6hIAVgVQAtgtgHApQgOAig6A3QhIBHhIAXg");
	this.shape_26.setTransform(40.616,-1.0717);

	this.shape_27 = new cjs.Shape();
	this.shape_27.graphics.f("#E4BE9D").s().p("Ah3BYQAeg1Ahg0QBDhsAVgDQAbgEAiAcQAjAcgKAWQgOAig7A3QhHBHhIAXg");
	this.shape_27.setTransform(39.5361,-3.7254);

	this.shape_28 = new cjs.Shape();
	this.shape_28.graphics.f("#003709").s().p("AgmAkIgRgCIgDgBQAPgDASgJQAjgRAPgfQATgHAPgBIgFAPIgBACIgBADIgBACIgCADIgBACIgCADIgBACIgCADIgBAAIgCADIgOAPIgDACIgHAHIgCABIgBABIgCABIgBABIgCABIgBABIgDABIgBAAIgDABIAAAAIgFABg");
	this.shape_28.setTransform(26.6,50.725);

	this.shape_29 = new cjs.Shape();
	this.shape_29.graphics.f("#004D1A").s().p("Ag5AtIgEgBQAVgDAXgNQArgbAGgwQALAEALAGIAIADQgXBShFAAQgNAAgOgDgAA+giIAAAAIAAAAg");
	this.shape_29.setTransform(26.875,49.4897);

	this.shape_30 = new cjs.Shape();
	this.shape_30.graphics.f("#004D1A").s().p("AgJAlQgsgFgugdQBJAAB9goQgXBLhEAAIgRgBg");
	this.shape_30.setTransform(22.9,50.5371);

	this.shape_31 = new cjs.Shape();
	this.shape_31.graphics.f("#357729").s().p("AgPAxQgdgFgfgPIgZgOQBGhGBIAFQAlADAWARQgXBRhGAAQgLAAgMgCg");
	this.shape_31.setTransform(22.925,49.2667);

	this.shape_32 = new cjs.Shape();
	this.shape_32.graphics.f("#357729").s().p("AgMgBIAZgKIgLAXg");
	this.shape_32.setTransform(32.75,9.65);

	this.shape_33 = new cjs.Shape();
	this.shape_33.graphics.f("#357729").s().p("AAHgJIAHAOIgbAEg");
	this.shape_33.setTransform(38.75,30.65);

	this.shape_34 = new cjs.Shape();
	this.shape_34.graphics.f("#357729").s().p("AAJgOIAJAPIgiAOg");
	this.shape_34.setTransform(22.95,5.05);

	this.shape_35 = new cjs.Shape();
	this.shape_35.graphics.f("#357729").s().p("AgDgKIAQAMIgZAJg");
	this.shape_35.setTransform(14.9,-5.725);

	this.shape_36 = new cjs.Shape();
	this.shape_36.graphics.f("#357729").s().p("AgMAAIAYgPIgHAfg");
	this.shape_36.setTransform(10.8,-18.075);

	this.shape_37 = new cjs.Shape();
	this.shape_37.graphics.lf(["#F0A293","#ED8881","#EA676B","#E95B63"],[0,0.31,0.757,1],-39.7,10.3,6.1,-3.5).s().p("Ag/BUQAfhSgUhWQA6AFAjAZQASAMAGALQgWAngXAWQghAmgvAQIgCAAg");
	this.shape_37.setTransform(-13.6,-25.45);

	this.shape_38 = new cjs.Shape();
	this.shape_38.graphics.lf(["#F0A293","#ED8881","#EA676B","#E95B63"],[0,0.31,0.757,1],-21.7,-0.2,21.2,0.3).s().p("ABbCHIACADQAigjAQglIgEAKIgSAfIgFAGIgKANQgJAMgQAOIgKALIgFADQANgNAMgSgAgGAzQgShKg3hJQghgsgegZQCJACBMDoQgjgPgqgDg");
	this.shape_38.setTransform(-18.275,-38.975);

	this.shape_39 = new cjs.Shape();
	this.shape_39.graphics.lf(["#E98E80","#E88077","#E45E61","#E35159"],[0,0.188,0.714,1],-22.3,-0.2,20.6,0.3).s().p("ABUDSIgBgCQAJgLALgUQgCgEgGgIQgNgNgUgKQgnh2g2g7QgpgsgvgJQgMgEgSAAIgCgCQABgbgBgWQAVgzAngLQAMgEABgBIAUABIAAAAIAEAAIABAAQAxAJBCBQQAlAtAaAxQAbAvAAAbIAAACQgCAqgPArQgJATgKARQgLASgVAVg");
	this.shape_39.setTransform(-17.6,-46.25);

	this.shape_40 = new cjs.Shape();
	this.shape_40.graphics.f("#FFFFFF").s().p("Ag3CUQgMgHABgSQADglA/g9QAGgGAVgxQAag7AGgmIAFgVQAJAPgWBFQgWBIgeAbQgyAsgFAlQgCATAIAJIgCAGg");
	this.shape_40.setTransform(-40.0962,-42.95);

	this.shape_41 = new cjs.Shape();
	this.shape_41.graphics.lf(["#E98E80","#E88077","#E45E61","#E35159"],[0,0.188,0.714,1],-5.1,-14.8,3.9,7.3).s().p("AgjhAQAngCARA1QAOAsABAeIgCAFQgGhbg/gng");
	this.shape_41.setTransform(-36.225,-67.2036);

	this.shape_42 = new cjs.Shape();
	this.shape_42.graphics.lf(["#F0A293","#ED8881","#EA676B","#E95B63"],[0,0.31,0.757,1],-0.7,0,3,0).s().p("AADAaQgKAAAAgCQADgYAKgZQACAWgCAbIACACIgBAAIgEAAg");
	this.shape_42.setTransform(-33.3977,-58.0875);

	this.shape_43 = new cjs.Shape();
	this.shape_43.graphics.lf(["#F0A293","#ED8881","#EA676B","#E95B63"],[0,0.31,0.757,1],-3.1,-10.4,5.9,11.7).s().p("AA1AZIAAgDQgSgugmAAQgdgSgrgFQAFgOANgMIACgBIABAAQAKgLAUgFIASgHQAtgGARAwIABADIABACQAGAVAEAjIAGA8IACASIgIAPQgCghgNgpg");
	this.shape_43.setTransform(-39.45,-71.2138);

	this.shape_44 = new cjs.Shape();
	this.shape_44.graphics.lf(["#F0A293","#ED8881","#EA676B","#E95B63"],[0,0.31,0.757,1],-6.2,-2.2,24.3,11.6).s().p("AgzAJIAEgDQASgRATgGQAdgJAeAOIAHADQgNAJgFAQQgegEgjAFIgcAFg");
	this.shape_44.setTransform(-51,-77.6642);

	this.shape_45 = new cjs.Shape();
	this.shape_45.graphics.lf(["#DF7165","#DC5053","#DB414B"],[0,0.627,1],-11.8,3.1,6.6,-2.4).s().p("AhHAeQgEgygEgaQAZAhAgADQAnAEAVAIQAcAKANAUIABACIAAACQgjgcgbgFIgBAAIgKgBIgPAAIgIADIgBAAIgCABQggAJgSAgIgCgRg");
	this.shape_45.setTransform(-24.875,-67.45);

	this.shape_46 = new cjs.Shape();
	this.shape_46.graphics.lf(["#E79688","#E68D81","#E47E75"],[0,0.314,1],-10.3,2.3,9,-1.8).s().p("AA7AiIAAgCIgBgCQgMgUgdgKQgVgIgngEQgngEgWghIgIgoIAQANIACACIAAACIAFAEQAjAiA6AXIAGAAQAsATAaAbQAaAaAHAcQgZgegdgZg");
	this.shape_46.setTransform(-22.375,-67.15);

	this.shape_47 = new cjs.Shape();
	this.shape_47.graphics.lf(["#E48072","#E1585B","#E04952"],[0,0.659,1],-11.1,0.8,7.3,-4.7).s().p("AhNhUIAHAFIAKBfQASggAfgKIADgBIABAAIAHgBIAbAAQAbAFAjAcQgDAZisA3IgBABg");
	this.shape_47.setTransform(-25.3,-64.2);

	this.shape_48 = new cjs.Shape();
	this.shape_48.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0,0.416,0.792,1],-2.3,-2.4,2.4,9.1).s().p("AgugRQAqAGAbARQgJAAgKADIgQAHQgLgGgTAFQgDAAgEACQgDgTAGgPgAAXAGIAHgBQAsABgyAAIgBAAgAAXAGIAAAAg");
	this.shape_48.setTransform(-42.3732,-74.35);

	this.shape_49 = new cjs.Shape();
	this.shape_49.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0,0.416,0.792,1],-16.2,8.3,29.6,-5.5).s().p("AASBMQgPgEhfgmQgsgSgfgHIgWgDQBagnBOgXQB3glBUAFQAUBXgfBRIABAAIACgBIgFACQgdAJgiAAQgpAAgvgOg");
	this.shape_49.setTransform(-37.1578,-24.9803);

	this.shape_50 = new cjs.Shape();
	this.shape_50.graphics.f("#FFFFFF").s().p("AgkAZQAHguAjg0QAOgVATgKQgXAdgYApQgkA5AQAvQAJAYAPALQgpgYAJg4g");
	this.shape_50.setTransform(-51.6737,-51.5);

	this.shape_51 = new cjs.Shape();
	this.shape_51.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0,0.416,0.792,1],-14.6,-0.1,28.3,0.4).s().p("AhyCMIhXgrQgLgHABgSQACglA/g7QBAhegEg7QAAABAQAAQAhAeAeAnQA2BJASBKQAvADAfAPIAJAcQAGAXAHAOQgQAWgJAJIAFgDQAEgDAGgIQAJgIAQgSIAKgNIAFgGIARgeQgkBUhIAWQgUAGgRABIgQAAQhJAAhcgmg");
	this.shape_51.setTransform(-25.479,-37.8535);

	this.shape_52 = new cjs.Shape();
	this.shape_52.graphics.lf(["#E48072","#E1585B","#E04952"],[0,0.659,1],-7.6,5.8,12.7,-8.9).s().p("AhbAiQgCgHACgOQAWADAkgLQApgMBVgoIgLAuQhBAag1AQIgSAFQgJACgHAAQgRAAgEgOg");
	this.shape_52.setTransform(-53.85,-27.8754);

	this.shape_53 = new cjs.Shape();
	this.shape_53.graphics.lf(["#FBD6CA","#FACDC2","#F7ACA7","#F6A09D"],[0,0.137,0.694,1],0.8,-0.7,17.1,-13.8).s().p("AgZAGIADgXIAEgFIACAIQAEAJASAHQAUAHAAAGQgBAEgJACIgEABIgNABQgVAAgDgRg");
	this.shape_53.setTransform(-61.725,-29.4022);

	this.shape_54 = new cjs.Shape();
	this.shape_54.graphics.lf(["#F0A293","#ED8881","#EA676B","#E95B63"],[0,0.31,0.757,1],-8.6,6.2,7.7,-6.9).s().p("AhMAWIAJgPICSgnIiVBAIgFABQgHAAAGgLg");
	this.shape_54.setTransform(-49.4945,-32.9306);

	this.shape_55 = new cjs.Shape();
	this.shape_55.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0,0.416,0.792,1],-7.7,4.9,8.6,-8.2).s().p("AiDA0QgOgWAUgYIAWgTIDDgyIAlAJQAaAPg5AcQhrAzgwAPQgdAJgSAAQgUAAgHgMg");
	this.shape_55.setTransform(-51.4463,-32.8973);

	this.shape_56 = new cjs.Shape();
	this.shape_56.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0,0.416,0.792,1],-6.6,-4.2,14.2,-2.8).s().p("AhUACIAVgUQAlgKBWgIIA0AEQgZAWhlAZQg0AOguAIQAFgMAXgXg");
	this.shape_56.setTransform(-46.2,-36.7);

	this.shape_57 = new cjs.Shape();
	this.shape_57.graphics.lf(["#A55C52","#A34041","#A23138"],[0,0.58,1],-6.3,-2.6,12.8,4.3).s().p("AgvASQAAgrAQggQALgUAYgUQAbgWAHAMQAUArgWBSQgUBQgbAOQgkgjAAg7g");
	this.shape_57.setTransform(-35.1309,-47.1264);

	this.shape_58 = new cjs.Shape();
	this.shape_58.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0,0.416,0.792,1],-5.4,-2.3,13.7,4.6).s().p("AgUBuQgWgngDgvQgDgqAPgnQALgbAYgSQAPgKAPgEIACgBIABAAQAHAHAEAIQg2AYgOAaQgQAfAAAsQAAA7AkAjIgDABIgCAAQgIAAgFgIg");
	this.shape_58.setTransform(-35.875,-47.6023);

	this.shape_59 = new cjs.Shape();
	this.shape_59.graphics.f("#F05D60").s().p("AAvA5QgjgRgkgkQgngkAEgQQADgOAaAFQANACAMAFIArA+QAgAwgOAAQgDAAgGgDg");
	this.shape_59.setTransform(-39.3497,-43.4558);

	this.shape_60 = new cjs.Shape();
	this.shape_60.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0,0.416,0.792,1],-9.1,-6.2,6.1,4.9).s().p("AgwBJIgJgQQgLgUgFgfQgFggAFgWQAFgVALgDIAFAAQATACBLAAIAiB5Ig/ANQgyAJgJAAIgCAAgAhAgEQAEAwASgGQATgHgSgqIgWgqg");
	this.shape_60.setTransform(-43.475,-44.4534);

	this.shape_61 = new cjs.Shape();
	this.shape_61.graphics.lf(["#E48072","#E1585B","#E04952"],[0,0.659,1],-0.7,-3.8,1.2,6.3).s().p("AACAPQgigXgFgOIAAgFQAEgEAEgBQAOgEAZARQANAIAKAJIACAKQACAOABAMQgPgDgVgQg");
	this.shape_61.setTransform(-39.1,-57.3806);

	this.shape_62 = new cjs.Shape();
	this.shape_62.graphics.lf(["#FBD6CA","#FACDC2","#F7ACA7","#F6A09D"],[0,0.137,0.694,1],-5.7,-3.6,9.5,7.6).s().p("AgPADIAAgwIAWApQASArgTAHIAAAAIgEAAQgOAAgDgrg");
	this.shape_62.setTransform(-48.4187,-45.4264);

	this.shape_63 = new cjs.Shape();
	this.shape_63.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0.227,0.549,0.839,1],1.2,7.6,1.8,-5.3).s().p("AACArQgngbgFgQQgCgKALgfQAIgVAhATQARAJAPANIADAvQgDAbgPAAQgJAAgOgKg");
	this.shape_63.setTransform(-39.6731,-55.3836);

	this.shape_64 = new cjs.Shape();
	this.shape_64.graphics.f("#A34442").s().p("AhABcIgXgUQgHgVgBgUQgCgnAcACQAtAEAQgCQgIgGgHgKQgMgTAKgTQARgfANgFQAYgIArAlQApAjgcAgQgyApgfAcQgcAZgXAAQgJAAgIgEg");
	this.shape_64.setTransform(-42.0582,-52.384);

	this.shape_65 = new cjs.Shape();
	this.shape_65.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0,0.416,0.792,1],-1.7,11.6,8.1,-9.3).s().p("AABB1QhSgLgChBQgBgyAuhEQAfguArAGQAWADAQAMIAGAlQAFArgCAjQgFBphCAAIgLgBg");
	this.shape_65.setTransform(-47.2351,-50.9981);

	this.shape_66 = new cjs.Shape();
	this.shape_66.graphics.f("#FFFFFF").s().p("AgnAOQAIgfAagmQAVggAcgHQgEAGgGADQg3AqgMBHQgGAkAEAbIgEAEQgJgrAJgmg");
	this.shape_66.setTransform(-51.5375,-61.8);

	this.shape_67 = new cjs.Shape();
	this.shape_67.graphics.lf(["#FBD6CA","#FACDC2","#F7ACA7","#F6A09D"],[0,0.137,0.694,1],-2.5,-4.4,9.4,18.5).s().p("AAjAVQgHgUgQgEIgOgBIgUAGQgTAGgHgEIgEgEQAHgWAcgIQAHgDAHAAIAFAAQARgCAMAKQAGAFADAFQALARABARQABAOgGACIgBAAQgGAAgFgOg");
	this.shape_67.setTransform(-40.6929,-67.86);

	this.shape_68 = new cjs.Shape();
	this.shape_68.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0,0.416,0.792,1],-3.6,-10.7,8.3,12.2).s().p("AgwgEQgOgUgDgXQgGguAzgPIAAABQA5gRAUBMQARA+gNBhg");
	this.shape_68.setTransform(-40.2301,-61.248);

	this.shape_69 = new cjs.Shape();
	this.shape_69.graphics.lf(["#F0A293","#ED8881","#EA676B","#E95B63"],[0,0.31,0.757,1],-4.9,-2.1,10.5,4).s().p("Ag4BnIgEgSQAbhhAvgOQAHgCADAAQgHgPgCgSQgDgiAbgMQANgFAJAJIAAABQgZB7gbBIQgPAKgPAFQgHACgHAAQgMAAgJgHg");
	this.shape_69.setTransform(-49.25,-60.6719);

	this.shape_70 = new cjs.Shape();
	this.shape_70.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0,0.416,0.792,1],-3.8,-1.6,7.6,2.9).s().p("Ag3A1QABg6Ang0QATgdAbgHQAPgEAJAAQABAAAAABQAAAAAAAAQgBAAgBAAQgBAAgBAAQgcARAFAjQACARAIAOQgJgBgDABQgeAJgXApQgPAcgHAhQgHgPAAgeg");
	this.shape_70.setTransform(-50.3654,-61.825);

	this.shape_71 = new cjs.Shape();
	this.shape_71.graphics.lf(["#F0A293","#ED8881","#EA676B","#E95B63"],[0,0.31,0.757,1],-5.4,-15.1,3.6,7).s().p("AhCgsQgHgRAAgLIAHgCQARgFAOAFIARgHQAHgDAMAAQA/AlAIBeIACgGQACAegHAOQhihUglgtg");
	this.shape_71.setTransform(-39.9278,-65.35);

	this.shape_72 = new cjs.Shape();
	this.shape_72.graphics.lf(["#FBD6CA","#FACDC2","#F7ACA7","#F6A09D"],[0,0.137,0.694,1],-3.3,-1.9,27.3,11.9).s().p("AgQAnQgMgyARgmIATgbQANgDABABIgGALQgSAhAFAuQADAYgBAVQgBAUgEACIAAAAQgHAAgJgog");
	this.shape_72.setTransform(-60.5423,-63.3833);

	this.shape_73 = new cjs.Shape();
	this.shape_73.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0,0.416,0.792,1],-8.9,-5.7,21.7,8.1).s().p("Ag2B4Qg6hwAWhEQAKgeA8hAIADAAIgDADIgFANIAcgFQAigFAgAEQAFgPANgLIAKAFIgtCLQgDAtgIAwQgRBfgaAIIgFABQgVAAgagzg");
	this.shape_73.setTransform(-54.0525,-61.5317);

	this.shape_74 = new cjs.Shape();
	this.shape_74.graphics.lf(["#E68476","#E57B71","#E2595C","#E14C54"],[0,0.133,0.694,1],-12.4,-1.9,11.9,5).s().p("AhPBWQgGgSAlgLQAPgEAPgCQAIgMAMgIQAJgGAFgBIABAAQg2gkgSgtIgIglIArAcQAvAeAbAPIAbAsIgIAkQACAAgVAIIguAPQggAKgYADQgVgFgKgEg");
	this.shape_74.setTransform(-53.8669,-41.975);

	this.shape_75 = new cjs.Shape();
	this.shape_75.graphics.lf(["#EC9587","#EA857C","#E76266","#E6555D"],[0,0.216,0.725,1],-8.6,-2.1,20,6).s().p("AhTBGQgCgLAAgTIABgsQAChJBHgWQAMgDANgBIAKgBIA+BnQgagQgwgfIgqgcIAIAlQATAsA1AlIAAAAQgYAJgMATQgOAAgOAFQgkALAEATQgdgMgIgXg");
	this.shape_75.setTransform(-57.1,-43.7333);

	this.shape_76 = new cjs.Shape();
	this.shape_76.graphics.f("#004D1A").s().p("AhUATIAWg/IBegoIgOArIBQgBQgPAOgfAUIgcAQQAWARAKALQgeAPgdAKQgvAPgvAJg");
	this.shape_76.setTransform(-1.925,-29.5);

	this.shape_77 = new cjs.Shape();
	this.shape_77.graphics.f("#357729").s().p("AgcBNIgJgaQgUAZgtAkIAXh6IAWhBIBegnIgOArIBQgBQgPAOgfAUIgcARQArAeAfAnQgpAFgjgLIgagMQAHAtgFAZQgBAMgDADQgQgKgLgcg");
	this.shape_77.setTransform(-2.5,-26.45);

	this.shape_78 = new cjs.Shape();
	this.shape_78.graphics.f("#003709").s().p("AATA2Ig8g5IgSgzQAQAMAOANIACADIAcAaQAQAMAPAQIAcAaIgSABIgXgBg");
	this.shape_78.setTransform(2.6,-24.9);

	this.shape_79 = new cjs.Shape();
	this.shape_79.graphics.f("#004D1A").s().p("AgJAvQgtgmgYgRIgHgFIAQguIAKgEIAPALQAmAeAgAeQAaAWAiAhIgXABQgpAAgfgRg");
	this.shape_79.setTransform(0.05,-25.8118);

	this.shape_80 = new cjs.Shape();
	this.shape_80.graphics.f("#003709").s().p("AAAAVQgRgVgQgSQAZAAALgDIAeAjIABAIg");
	this.shape_80.setTransform(16.85,-8.1);

	this.shape_81 = new cjs.Shape();
	this.shape_81.graphics.f("#004D1A").s().p("AgLASIglgnQAjAGAfgGIAeAjIABAIg");
	this.shape_81.setTransform(15.425,-8.1);

	this.shape_82 = new cjs.Shape();
	this.shape_82.graphics.f("#002A00").s().p("AgkgnQAYAIAOAGIAjA2QgHAHgMAEQgjg1gTgag");
	this.shape_82.setTransform(30.75,10.475);

	this.shape_83 = new cjs.Shape();
	this.shape_83.graphics.f("#004D1A").s().p("AE3JaQgTiZglh7Qguiahgi+QhYixitjDQg3g9g4g4IgugrIgTgzQAfAZAfAcQBKBDA5BCQB5CIBeCnQBcCiA9CxQA5CmAeDCQABAGgDAEQgEAFgFAAg");
	this.shape_83.setTransform(29.1111,29.8);

	this.shape_84 = new cjs.Shape();
	this.shape_84.graphics.f("#004D1A").s().p("AACApQgSgfgdglIAKgOIAPAEIAeALQAYAjALATQgOAOgVAAIgIgBg");
	this.shape_84.setTransform(29.9,10.4737);

	this.shape_85 = new cjs.Shape();
	this.shape_85.graphics.f("#357729").s().p("AFwJ5QgEgEgBgFQggi4g9ipQg+itheibQheidh6iGQg/hFhDg5QgmgggggZIgkgaIgkgXQgLgGgCgLQgDgLAGgLQAFgLAMgDQALgDALAGIAUAMQAKAHAJAIQANAJAZAUQAWAQAyAtQBJBCA6BDQB7CNBcChQBbCfA9CzQA6CoAdDAQABAGgDAEQgEAFgFAAIgCAAQgFAAgDgCg");
	this.shape_85.setTransform(22.4125,26.3861);

	this.shape_86 = new cjs.Shape();
	this.shape_86.graphics.f("#003709").s().p("AhDBgQAignAcgsQAqhDALg4IACgBIAKAZQAKAfgCAeQgHBeh6ArIgGgQg");
	this.shape_86.setTransform(29.1424,-12.7);

	this.shape_87 = new cjs.Shape();
	this.shape_87.graphics.f("#003709").s().p("AgVgWQAbAAAVgKQgTAfgXAIIAYAPQgSAJgPACQgEgdAHgag");
	this.shape_87.setTransform(24.6773,-7.25);

	this.shape_88 = new cjs.Shape();
	this.shape_88.graphics.f("#004D1A").s().p("AhFBIQgDgzAlgyQASAQARgHQAdgNAVhNIACgBIAKAZQAKAfgCAeQgHBeh5ArQgJgPgCgZg");
	this.shape_88.setTransform(28.9351,-12.7);

	this.shape_89 = new cjs.Shape();
	this.shape_89.graphics.f("#357729").s().p("AhDAuQAMhRBrhNIAKAZQAKAggCAdQgHBeh5AsQgPgZAGgpg");
	this.shape_89.setTransform(28.9413,-12.65);

	this.shape_90 = new cjs.Shape();
	this.shape_90.graphics.f("#003709").s().p("AgfAhQgIAAgEgCQAnAAAYg2IAYgJIgGANIgIAPIgEAGIgCADIgEADIgCADIgJAKIgDACIgDABIgDACIAAABIgFABIgCABIgDABIgDABQAAABAAAAQgBAAAAAAQgBABAAAAQgBAAgBAAg");
	this.shape_90.setTransform(40.15,41.075);

	this.shape_91 = new cjs.Shape();
	this.shape_91.graphics.f("#004D1A").s().p("AgqAkQAqgBAdhIQAIAFAGAFQgNApgaAPQgNAJgSAAIgPgCg");
	this.shape_91.setTransform(40.3,40.4735);

	this.shape_92 = new cjs.Shape();
	this.shape_92.graphics.f("#004D1A").s().p("AgJAgQgegDgggVQAugGA1gRQAagJASgIQgVBBgzAAIgJgBg");
	this.shape_92.setTransform(37.475,41.0234);

	this.shape_93 = new cjs.Shape();
	this.shape_93.graphics.f("#357729").s().p("AgNAsQgUgDgVgMQgRgJAAgCQAkhFA5AIQAdAEAVASQgVBCgzAAIgNgBg");
	this.shape_93.setTransform(37.425,39.8874);

	this.shape_94 = new cjs.Shape();
	this.shape_94.graphics.f("#F4D4CD").s().p("AgUAEQAHgEAHAAQAOgFANABIgHACIgIACQgNADgEADg");
	this.shape_94.setTransform(33.375,-89.2083);

	this.shape_95 = new cjs.Shape();
	this.shape_95.graphics.f("#357729").s().p("AgGgBIANgCIgLAHg");
	this.shape_95.setTransform(32.475,30.025);

	this.shape_96 = new cjs.Shape();
	this.shape_96.graphics.f("#357729").s().p("AgIgGIARACIgOALg");
	this.shape_96.setTransform(30.175,16.3);

	this.shape_97 = new cjs.Shape();
	this.shape_97.graphics.f("#357729").s().p("AgGgFIANAFIgNAGg");
	this.shape_97.setTransform(28.925,-5.025);

	this.shape_98 = new cjs.Shape();
	this.shape_98.graphics.f("#357729").s().p("AgMAKIATgUIAFAVg");
	this.shape_98.setTransform(35.55,54.45);

	this.shape_99 = new cjs.Shape();
	this.shape_99.graphics.f("#357729").s().p("AgEADIAGgHIADAJg");
	this.shape_99.setTransform(31.55,41.15);

	this.shape_100 = new cjs.Shape();
	this.shape_100.graphics.f("#357729").s().p("AgQABIAYgNIAIAZg");
	this.shape_100.setTransform(28.3,32.85);

	this.shape_101 = new cjs.Shape();
	this.shape_101.graphics.f("#004D1A").s().p("AgIABIARgGIgDALg");
	this.shape_101.setTransform(25.325,15.1);

	this.shape_102 = new cjs.Shape();
	this.shape_102.graphics.f("#357729").s().p("AgFgDIALgBIAAAJg");
	this.shape_102.setTransform(23.8,-7.025);

	this.shape_103 = new cjs.Shape();
	this.shape_103.graphics.f("#004D1A").s().p("AgMgFIAYgBIABANg");
	this.shape_103.setTransform(23.35,-14.85);

	this.shape_104 = new cjs.Shape();
	this.shape_104.graphics.f("#003709").s().p("AhbA6IACgJQBigtA/gyIAUgNQgCAvgeAfQgmAphKAAQgSAAgVgCg");
	this.shape_104.setTransform(37.625,4.6461);

	this.shape_105 = new cjs.Shape();
	this.shape_105.graphics.f("#FAC7BD").s().p("AAEARQgvgLgIgcIAjAFQAmALAeAdQgXAAgZgGg");
	this.shape_105.setTransform(38.7,-60.3);

	this.shape_106 = new cjs.Shape();
	this.shape_106.graphics.f("#FBD3C8").s().p("AgEA3QgLgHAFg0IAIg0QAQAqgCAoQgCAfgJAAQgCAAgDgCg");
	this.shape_106.setTransform(45.0017,-66.507);

	this.shape_107 = new cjs.Shape();
	this.shape_107.graphics.f("#FBD4CC").s().p("AgeCJQgIgGADhXIAGiNQABgdADgJIACgEQALAhAYAUQANAKALADQgfApgMBlIgIA4QgDAPgGAAQgDAAgDgDg");
	this.shape_107.setTransform(20.9091,-50.8982);

	this.shape_108 = new cjs.Shape();
	this.shape_108.graphics.f("#FFFFFF").s().p("Ag6gOQAbgUAogFQAygFAlAaQgYgOgwgCQg1gDgmAmIgcAoQAGgjAfgUg");
	this.shape_108.setTransform(31.775,-73.0903);

	this.shape_109 = new cjs.Shape();
	this.shape_109.graphics.lf(["#F0A293","#ED8881","#EA676B","#E95B63"],[0,0.31,0.757,1],-26,-7.5,5.4,1.6).s().p("AgvgUIADgnQAZAFAbgDQAbgCAWgHIASgGQgYAfgiAiQgvAwgsAgQAUguAHgvg");
	this.shape_109.setTransform(46.3,-45);

	this.shape_110 = new cjs.Shape();
	this.shape_110.graphics.lf(["#EC9888","#EA827A","#E76266","#E6565E"],[0,0.278,0.741,1],-24.4,-7,9,2.7).s().p("Ah4CFQAxgyAcg/QBDgwBShgIAMgIIAEADQgLARgOApQgeBcglA0QgaAlgfAMIgBABIgDACIgEABIgVAEIgYACIgeABg");
	this.shape_110.setTransform(43.15,-39.875);

	this.shape_111 = new cjs.Shape();
	this.shape_111.graphics.lf(["#F4AFA0","#F18D88","#ED6C72","#EC6069"],[0,0.365,0.773,1],-20,-6.9,13.4,2.8).s().p("AhFCFQgeAAgUgKQgSgJgMgLQgMgLgHgOQAGgFADgBQALgDAYABQARACAbgIQA8gSBPhIQAwgrAug4IAIgFQAEgDADACQAAAAAAAAQABABAAAAQAAAAAAAAQAAAAgBABQgKASgNAnQg4CohGAZIgJAEIgCABQgYAHgxAAg");
	this.shape_111.setTransform(38.67,-39.8);

	this.shape_112 = new cjs.Shape();
	this.shape_112.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0,0.416,0.792,1],-17.9,-4.5,12.6,4.3).s().p("AidBgQgGgYAAgMQgEgqAKgjQAThBA9gTIAdgHIAOACQAVAEAgAOQAZAMAmgBQAiAAAggKIASgGQgsA2gyAsQhPBJg8ASQgXAIgUgCQgYgBgMADQgGACgCADIgDgNg");
	this.shape_112.setTransform(37.2948,-43.0875);

	this.shape_113 = new cjs.Shape();
	this.shape_113.graphics.f("#FFFFFF").s().p("ABuBhQgIgQgvgYQg/ggglgUQgngUgVgkQgMgXgFgZIAEgFIABAAQAMAwAlAgQAbAWBEAiQBRAoAPASIgGAPQgDAAgEgIg");
	this.shape_113.setTransform(29.45,-60.225);

	this.shape_114 = new cjs.Shape();
	this.shape_114.graphics.f("#F9BBB0").s().p("AhkCiQgigjADgrIAFg7QACgnADhUQAChSAGgcIAGgMQAMAwAmAgQAaAXBFAiQAxAYAcATQAOAKAEAFIg0CfIgtBXQhdgQgrgrg");
	this.shape_114.setTransform(28.5413,-48.65);

	this.shape_115 = new cjs.Shape();
	this.shape_115.graphics.lf(["#EE9181","#EC8177","#E95F63","#E8525B"],[0,0.212,0.722,1],2.2,-8.2,-1.5,6).s().p("AgggIIAEgGQACgEADAAQAhAMAXAXIgHACQgVgSglgJg");
	this.shape_115.setTransform(47.875,-52.6528);

	this.shape_116 = new cjs.Shape();
	this.shape_116.graphics.lf(["#F7A797","#F48B84","#F16A6E","#F05E66"],[0,0.325,0.761,1],1.5,-5.8,-2.2,8.4).s().p("AgiAbQgSgKgdgPIgOgGIgKg8IALABIAXA4IARAHQASAHAHAKIAuADQA2AJAjAjIgHABQglgihggEg");
	this.shape_116.setTransform(40.575,-57.225);

	this.shape_117 = new cjs.Shape();
	this.shape_117.graphics.lf(["#F0A293","#ED8881","#EA676B","#E95B63"],[0,0.31,0.757,1],2.1,-7.1,-1.6,7.1).s().p("AgcgIIANgSQAZAbATAaQgZgYgggLg");
	this.shape_117.setTransform(48.35,-53.7);

	this.shape_118 = new cjs.Shape();
	this.shape_118.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0,0.416,0.792,1],0.9,-5.7,-2.8,8.5).s().p("AALAUIgugDQgHgKgSgHIgRgHIgWg4IAYABQApAEAbAOQAaAOArAwQAWAYAQAWQgjgjg2gJg");
	this.shape_118.setTransform(41.2125,-57.325);

	this.shape_119 = new cjs.Shape();
	this.shape_119.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0.227,0.549,0.839,1],-4.8,-1.2,8.9,0.1).s().p("AgWgmQABgMgJgLIgJgIIAOgGQAQgGAOAEQAOAEALAcQAGAOACAOQAGAlgOA1Ig+AGg");
	this.shape_119.setTransform(45.8026,-67.6931);

	this.shape_120 = new cjs.Shape();
	this.shape_120.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0,0.416,0.792,1],5.4,1.9,-2.8,-2.3).s().p("AAEAYQgRgIgKgCQAFgRAIgOQAKgUANAHIADACQAHAJAAALQABANgIAcIgMgJg");
	this.shape_120.setTransform(40.0016,-64.2797);

	this.shape_121 = new cjs.Shape();
	this.shape_121.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0.227,0.549,0.839,1],1.6,3.8,-1.3,-3.4).s().p("AAnAeQgggQhAgFQATgkAPgCQA/gHAKANQASAZgXAeg");
	this.shape_121.setTransform(36.5824,-64.8339);

	this.shape_122 = new cjs.Shape();
	this.shape_122.graphics.f("#F05D60").s().p("AAHARIgUgMQgCgLABgHQABgOAPAXQARAVgEACg");
	this.shape_122.setTransform(31.3247,-64.2185);

	this.shape_123 = new cjs.Shape();
	this.shape_123.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0,0.416,0.792,1],-4.4,4.9,3.4,-3.4).s().p("AgcAtIgChbQgBgIACgGQAEgMANAGQAUAKAYAbIgQBdIghAFg");
	this.shape_123.setTransform(30.0938,-63.5473);

	this.shape_124 = new cjs.Shape();
	this.shape_124.graphics.lf(["#F0A293","#ED8881","#EA676B","#E95B63"],[0,0.31,0.757,1],1.1,-5.9,-1.5,8.9).s().p("AgxAWQADgPAAgNIAAgKIAbgZQAnAJAUATQAQAPgMAOQgJAKgYAHQgNAEgRADIgOACg");
	this.shape_124.setTransform(46.4026,-49.3);

	this.shape_125 = new cjs.Shape();
	this.shape_125.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0,0.416,0.792,1],1.8,8.6,-2.8,-5.8).s().p("AhPgvIA7AEQA/AJAbAbQAbAagwARIg0AMg");
	this.shape_125.setTransform(43.511,-49.775);

	this.shape_126 = new cjs.Shape();
	this.shape_126.graphics.f("#FBCBBA").s().p("AhPgvIA7AEQA/AJAbAbQAbAagwARIg0AMg");
	this.shape_126.setTransform(43.511,-49.775);

	this.shape_127 = new cjs.Shape();
	this.shape_127.graphics.f("#A44F49").s().p("AhMhGIASACQATADAMANIAwgGQAyABAEAfQAHA2glAaIgnARg");
	this.shape_127.setTransform(35.8051,-63.05);

	this.shape_128 = new cjs.Shape();
	this.shape_128.graphics.f("#F2716A").s().p("AgOAuIAIhkIAVBug");
	this.shape_128.setTransform(27.8,-64.05);

	this.shape_129 = new cjs.Shape();
	this.shape_129.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0.122,0.486,0.816,1],-2.5,15.5,7.1,-15.4).s().p("AAYCCQhLgXglhEQgphLAggzQAcgtBDgGQAxgGAlAbQATAOAJAPIgTDkIgKAAQgbAAgggKg");
	this.shape_129.setTransform(33.0236,-63.1817);

	this.shape_130 = new cjs.Shape();
	this.shape_130.graphics.f("#E0A19C").s().p("AgXALQgfgcgggkIABgGIAMgPQAvgBAQAXIAlgKQAogBAKAsQAJAnABAXQgOAVgMAJIgIACQgSgEg6g8g");
	this.shape_130.setTransform(42.325,-71.15);

	this.shape_131 = new cjs.Shape();
	this.shape_131.graphics.lf(["#F0A293","#F0A293","#ED8881","#EA676B","#E95B63"],[0,0.227,0.471,0.812,1],2,-7.2,-12,29).s().p("AAHgHQgRgjghgdQArAOAUARQAVARADAbQADAcgXApQACgpgTgng");
	this.shape_131.setTransform(48.9423,-74.75);

	this.shape_132 = new cjs.Shape();
	this.shape_132.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0.227,0.549,0.839,1],6,24.7,-12.8,-9.6).s().p("AggAgIhChEIAcgiQAkghAmAMQAxAQAUAQQAYASACAdQADAagVAmQgSAigQAIIgCABQgPAAg+g/g");
	this.shape_132.setTransform(43.3691,-72.9875);

	this.shape_133 = new cjs.Shape();
	this.shape_133.graphics.lf(["#EB786A","#E85457","#E7454F"],[0,0.647,1],3.3,-0.1,-3.5,0.2).s().p("AAEgnIAMgIIgNBbQgKADgIABQAWgrgDgsg");
	this.shape_133.setTransform(50.875,-65);

	this.shape_134 = new cjs.Shape();
	this.shape_134.graphics.lf(["#E98E80","#E88077","#E45E61","#E35159"],[0,0.188,0.714,1],2.2,-4.2,-4.6,4.7).s().p("AASAqQgcgEgHgpIgDgqIgNAVIgCgIIgBgKIAOgHIACAAQAggKAQATQAJAKABALIgDAbQgHAggZANQAJgFAGgGg");
	this.shape_134.setTransform(54.9,-66.9833);

	this.shape_135 = new cjs.Shape();
	this.shape_135.graphics.f("#D14A4D").s().p("AgPgiIANgVIACAqQAIApAcADQgHAGgHAFQgFADgMAEQgLAEgdADQAYgsgEgug");
	this.shape_135.setTransform(53.05,-65.975);

	this.shape_136 = new cjs.Shape();
	this.shape_136.graphics.lf(["#EC998D","#EB8D85","#E76C6F","#E66067"],[0,0.169,0.706,1],5.9,-4.9,-0.9,4.1).s().p("AgwA4IAEgbIA4hVIACAqQAIAqAbADQgFAHgJAEQgEAEgNADQgRAGgdABg");
	this.shape_136.setTransform(51.6,-65.95);

	this.shape_137 = new cjs.Shape();
	this.shape_137.graphics.lf(["#EB786A","#E85457","#E7454F"],[0,0.647,1],6.6,-0.5,-6.9,0.7).s().p("AADBGIgJgFIgrgxQArgsAOgtIAqACIgJAlQgIArAAAhQAAAcgNADIgEABQgFAAgIgEg");
	this.shape_137.setTransform(49.475,-55.1964);

	this.shape_138 = new cjs.Shape();
	this.shape_138.graphics.lf(["#F48779","#F48578","#F05D60","#EF4E57"],[0,0.024,0.667,1],8.7,-0.6,-4.8,0.6).s().p("AAYBHIgJgFIhVhhIA4gqIBVADIgJAlQgIArABAhQAAAbgNADIgFABQgFAAgIgDg");
	this.shape_138.setTransform(47.275,-55.3442);

	this.shape_139 = new cjs.Shape();
	this.shape_139.graphics.lf(["#F4B0A2","#F4B0A2","#F1938E","#ED7175","#EC646C"],[0,0.227,0.475,0.816,1],-6.2,-8.3,5.3,5.1).s().p("Ag6BOIgNgdQgFgeALgcQAOgmAngRIAcgMQATgFAQAAQAHgBAPACIgDAIQgVAwgMAYQgdA5gXAXIgBAAQgJACgJAAQgMAAgMgEg");
	this.shape_139.setTransform(28.1583,-70.1154);

	this.shape_140 = new cjs.Shape();
	this.shape_140.graphics.lf(["#F9BEAE","#F9BEAE","#F5938F","#F17278","#F0666F"],[0,0.227,0.549,0.839,1],-5.1,-6.5,4.9,5.2).s().p("AhQgHQAEghAmgdQAJgIASgGQAcgIAjAJQASAFAMAGIgEAIIgWgBQgQABgTAGQgSAGgKAFQgnARgOAlQgLAcAFAfIAJAVQgcg+AFghg");
	this.shape_140.setTransform(27.7127,-71.8882);

	this.shape_141 = new cjs.Shape();
	this.shape_141.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0.227,0.549,0.839,1],-3.2,5.4,3.8,-7.8).s().p("AgtgwQABgfAZgGQANgEALADIApCuQheg3ADhRg");
	this.shape_141.setTransform(22.1204,-68.6442);

	this.shape_142 = new cjs.Shape();
	this.shape_142.graphics.f("#FBCBBA").s().p("AgtgwQABgfAZgGQANgEALADIApCuQheg3ADhRg");
	this.shape_142.setTransform(22.1204,-68.6442);

	this.shape_143 = new cjs.Shape();
	this.shape_143.graphics.lf(["#F0A293","#EE8982","#EC7574","#EA666B","#E95E65","#E95B63"],[0,0.133,0.286,0.463,0.671,1],2.6,-4,-3.2,1.8).s().p("AgNgoQANgDAIALQAGAGADAHQAEATgIAQQgEAIgEAFIgQANQgLguAJgkgAgOgoIABAAIAAAAg");
	this.shape_143.setTransform(55.9676,-56.7129);

	this.shape_144 = new cjs.Shape();
	this.shape_144.graphics.lf(["#DF7165","#DC5053","#DB414B"],[0,0.627,1],14.7,-22.1,-1.7,0.9).s().p("AhUDGIAXihIAWhLQAwhEgGhBIAMghIAbgCQAdACAJAWQAPAvggAuQAJgCAEAHQAIAOgMAqQgmB7gZAtQgXApgtAYIgTACQgFgDgBgGg");
	this.shape_144.setTransform(50.9503,-52.625);

	this.shape_145 = new cjs.Shape();
	this.shape_145.graphics.lf(["#F0A293","#ED8881","#EA676B","#E95B63"],[0,0.31,0.757,1],9.4,-20.2,-7,2.8).s().p("Ah0DzIABgJQAEAIACABIATgCQAtgZAXgoQAaguAlh7QAMgqgIgNQgEgHgJACQAhgvgQguQgIgXgdgCIgaACIgMAhQgFglgUgiIAEgaIAQgHQAigIAfAUQAoAaAZAhQAdAmgSATQgTATgRAoQgLAXghBXIgmBsQgcA9gsANQgPAFgOAAIgHgBg");
	this.shape_145.setTransform(54.0103,-56.1732);

	this.shape_146 = new cjs.Shape();
	this.shape_146.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0,0.416,0.792,1],9.4,-20.3,-7,2.8).s().p("Ah0DzIABgJQAEAHACACIATgCQAtgZAXgpQAagtAlh7QAMgqgIgOQgEgHgJADQAhgwgQgtQgIgXgdgDIgaADQgvCAgeBxIAskxIAQgHQAhgIAgAUQAoAZAZAhQAdAngSATQgTATgRAoQgMAYggBWIgmBrQgcA9gsAOQgPAEgOAAIgHAAg");
	this.shape_146.setTransform(54.0103,-56.1982);

	this.shape_147 = new cjs.Shape();
	this.shape_147.graphics.f("#FBCBBA").s().p("Ah0DzIBFnbIAYgJQAcgFAdASQAoAZAaAhQAcAmgSAUQgTATgRAoQgMAZggBVIgVA+QgNAngMAXQgiA+g7AAIgHAAg");
	this.shape_147.setTransform(54.0641,-56.2929);

	this.shape_148 = new cjs.Shape();
	this.shape_148.graphics.f("#FFFFFF").s().p("AgkgBQAcgyA+gFIgQAFQgwAQgVApIgMAnIgKAMQADgjAOgXg");
	this.shape_148.setTransform(30.2,-84.3);

	this.shape_149 = new cjs.Shape();
	this.shape_149.graphics.lf(["#F0A293","#ED8881","#EA676B","#E95B63"],[0,0.31,0.757,1],-0.3,-8.1,1.1,31.4).s().p("AgFgMQgsgbgsgKQAHgEAGgBQAQgFAwAEIAOAAQA2AKAYAwQAFALAJAbIAEAMIgWAFQgdgogwgeg");
	this.shape_149.setTransform(40.725,-83.8512);

	this.shape_150 = new cjs.Shape();
	this.shape_150.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0,0.416,0.792,1],2.2,30.6,-1.7,-8.9).s().p("Ah+AuQAAgHACgKQAEgVAJgRQAdg5BHgDQBIgCAiAqQAJAMAXA8Qg4APg7AIQgkAEgbAAQg9AAgOgYg");
	this.shape_150.setTransform(37.475,-82.9666);

	this.shape_151 = new cjs.Shape();
	this.shape_151.graphics.lf(["#EC998D","#EB8D85","#E76C6F","#E66067"],[0,0.169,0.706,1],-11.5,-14.1,3.5,3.6).s().p("AgOClQgHgJgFg1QALgiADhWQADhhAKgrIAJgYIARFMIgbAfIgOgRg");
	this.shape_151.setTransform(16.8,-56.775);

	this.shape_152 = new cjs.Shape();
	this.shape_152.graphics.lf(["#F9BEAE","#F5938F","#F17278","#F0666F"],[0,0.416,0.792,1],-11.7,-17.2,3.3,0.5).s().p("AAZD+Qg5gbgcgaQAKgLgQgpQgRgqgFAPIABATQgEglgChDQAAiGgBgYQgCguAMgkQARgyArgRIAVgGQAbgJATADQAJACAEACIAoIxQghgLgmgSgAhLC4QAGAJAJAIIAAAAQgKgHgFgKgAg8DJIAAAAg");
	this.shape_152.setTransform(23.0862,-58.849);

	this.shape_153 = new cjs.Shape();
	this.shape_153.graphics.f("#002A00").s().p("AAYAdQAAALgEAQQgegNgbgXIABgOIgNAEQgfgbgZgiQARgGAQgDIAhgCQAfABgJAPQAsAEAYAeQAjgNANARQALAOgGAHQgIAFgMgBQgLgBgLgEQALAagBAZQgqgUgGgOg");
	this.shape_153.setTransform(28.9841,-26.125);

	this.shape_154 = new cjs.Shape();
	this.shape_154.graphics.f("#003709").s().p("AgXATIABggIg5ARIgUAFQALgZANgVQgOgDgQgJIgMgIQALgMAugKIAhgCQAdABgHAPQAsAEAYAeQAigNAOARQAHAJABAKQgQAIgbgJQALAZgBAZQgngSgIgOIgBgJQAAAdgNAeQgGAPgIAIQgdgagFglg");
	this.shape_154.setTransform(27.8,-24.225);

	this.shape_155 = new cjs.Shape();
	this.shape_155.graphics.f("#002A00").s().p("AgdAbIAAgGQADgoAGgmIAEgZQAYAAAWAIQgEATgBAGQgEAYgHAzQgDAVgBAeIgGAGQgegbgDgdg");
	this.shape_155.setTransform(27.65,-19.7625);

	this.shape_156 = new cjs.Shape();
	this.shape_156.graphics.f("#002A00").s().p("AgjCMQgCgUAAgTQgBhUAFhJQADgqAGgmQAFgeACgJQAAgFACgFIA1gSIAAACIgFASIgEARIgHAnQgFAXgGA1QgJBIACBQQAAAuAFA5g");
	this.shape_156.setTransform(27.9089,-11.825);

	this.shape_157 = new cjs.Shape();
	this.shape_157.graphics.f("#003709").s().p("AgQgzIABgBIAXAGIAJBPQgLAMgIAIQgGgigIhGg");
	this.shape_157.setTransform(27.875,11.675);

	this.shape_158 = new cjs.Shape();
	this.shape_158.graphics.f("#004D1A").s().p("AgsCjQhmkjAJj4QAChOANhAIANgwIAPAOQgFAYgGA0QgIBPABBJQACCWAgCcQAgCXA2CRQAzCIBOCSQADAFgCACQhciNhakHg");
	this.shape_158.setTransform(39.6703,30.55);

	this.shape_159 = new cjs.Shape();
	this.shape_159.graphics.f("#004D1A").s().p("AgWgkIAOgVIAXAHIAIBOQgQAUgRAKQgHgtgFgxg");
	this.shape_159.setTransform(27.2,12.175);

	this.shape_160 = new cjs.Shape();
	this.shape_160.graphics.f("#357729").s().p("ACGJnQgEgBgDgEQhPiLg5iPQg7iTgkiYQgkiegGiZQgDhTAHhLQADgpAGgmIAHgnIAJgoQADgJAJgFQAJgFAJADQALABAGAIQAFAJgDAKIgRBIQgIAygDAaQgHBIAABRQABCSAhCfQAfCXA3CSQA3CSBKCIQACAEgBAFQgBAEgEACIgFACIgDgBg");
	this.shape_160.setTransform(38.7193,26.6567);

	this.shape_161 = new cjs.Shape();
	this.shape_161.graphics.f("#003709").s().p("AgVAUIgOgCIgBAAIgOgEIgNgGQBGAAAVgbQALABAWAGIADAGIgIAFIgJAEIgBABIgRAIIgBABIgIADIgCAAIgHACIgBAAIgIABIgBAAIgGABg");
	this.shape_161.setTransform(22.025,28.45);

	this.shape_162 = new cjs.Shape();
	this.shape_162.graphics.f("#004D1A").s().p("AhAAZQBAAAAWgWQAQgOgGgaQAPAQAKARQAHAMABACQgoAcglAAQgaAAgagNg");
	this.shape_162.setTransform(22.1,26.712);

	this.shape_163 = new cjs.Shape();
	this.shape_163.graphics.f("#004D1A").s().p("AgzAXQgZgWgTgfIgQgdIADAAQAbAXArAUQA9AbBWAOIADAGQgpAdgnAAQgqAAgpglg");
	this.shape_163.setTransform(17.35,24.4507);

	this.shape_164 = new cjs.Shape();
	this.shape_164.graphics.f("#357729").s().p("AgzAdQgZgWgTgfIgQgdQBkghBHA7QAkAcAQAkQgpAdgnAAQgqAAgpglg");
	this.shape_164.setTransform(17.35,23.869);

	this.shape_165 = new cjs.Shape();
	this.shape_165.graphics.f("#003709").s().p("AhJA8QBjhRAvgyIABAAQACAdgRAiQgjBAhfAQg");
	this.shape_165.setTransform(41.9105,34.3);

	this.shape_166 = new cjs.Shape();
	this.shape_166.graphics.f("#004D1A").s().p("AhAAFQAdhCBtgKQACAegRAhQgiBAhgAQQgHggAOgjg");
	this.shape_166.setTransform(41.8827,34.3);

	this.shape_167 = new cjs.Shape();
	this.shape_167.graphics.f("#004D1A").s().p("AhbA6QADgTAKgXQATgtAhgYQAAAcAVAGQAdAIBDgwQgBAvgeAfQgmAphKAAQgSAAgVgCg");
	this.shape_167.setTransform(37.65,4.6221);

	this.shape_168 = new cjs.Shape();
	this.shape_168.graphics.f("#003709").s().p("AhbBCQAGgqAagkQA0hKBjAZQACAlgWAiQglA6hZAAQgRAAgUgCg");
	this.shape_168.setTransform(37.6583,3.8301);

	this.shape_169 = new cjs.Shape();
	this.shape_169.graphics.f("#CC9579").s().p("AgWBLQgjgYgKgjQgKgiAQggQARghAqgUQAEAYAIALQgSAMgJAQQgtBIA+ArQAgAWAoAGQg5gDglgZg");
	this.shape_169.setTransform(37.8613,52.3);

	this.shape_170 = new cjs.Shape();
	this.shape_170.graphics.f("#CC9579").s().p("AhJBSQgjgZgIglQgJgjAUggQAWgjAvgQQBQgdAuAbQAXANAHATIgPAaQABgNgVgKQgUgKgcgBQhIgCgeAxQgtBHA/ArQAgAWAnAHQg6gFgngbg");
	this.shape_170.setTransform(42.5232,51.3116);

	this.shape_171 = new cjs.Shape();
	this.shape_171.graphics.f("#CC9579").s().p("AgqAvQgDgoAHgQQATgpA1gnQADAHAGAgIgbAWQgoAdgKAvQgFAYADASQgEgJgCgig");
	this.shape_171.setTransform(25.755,29.125);

	this.shape_172 = new cjs.Shape();
	this.shape_172.graphics.f("#CC9579").s().p("AhrBAQgDgmAHgSQAMgXAUgUQAYgZArgYQArgaAhAJQARAEALAMQAKAIgCAIQAAAEgDADQgYgVgtAYQg1AlgcASQgrAYgKAtQgFAXAEARQgFgGgDgjg");
	this.shape_172.setTransform(32.1545,27.0438);

	this.shape_173 = new cjs.Shape();
	this.shape_173.graphics.f("#CC9579").s().p("ABMBaQgGg4AMgSQAdgogigqQgTgXgbgIQgfgKgwAKQgrAIgJAhQgEAQAEAOQBKAXA+gLQAggGARgLQgKAPgyAWIAgANQgvALgqgCIghgEQhOgbAHg4QACgRAKgSQAFgJAFgGQAagkBQAEQBVAEAmA1QAiAwgQBDQgIAjgPAYIgaA0QgFgYgDgcg");
	this.shape_173.setTransform(18.6772,-6.9448);

	this.timeline.addTween(cjs.Tween.get({}).to({state:[{t:this.shape_173},{t:this.shape_172},{t:this.shape_171},{t:this.shape_170},{t:this.shape_169},{t:this.shape_168},{t:this.shape_167},{t:this.shape_166},{t:this.shape_165},{t:this.shape_164},{t:this.shape_163},{t:this.shape_162},{t:this.shape_161},{t:this.shape_160},{t:this.shape_159},{t:this.shape_158},{t:this.shape_157},{t:this.shape_156},{t:this.shape_155},{t:this.shape_154},{t:this.shape_153},{t:this.shape_152},{t:this.shape_151},{t:this.shape_150},{t:this.shape_149},{t:this.shape_148},{t:this.shape_147},{t:this.shape_146},{t:this.shape_145},{t:this.shape_144},{t:this.shape_143},{t:this.shape_142},{t:this.shape_141},{t:this.shape_140},{t:this.shape_139},{t:this.shape_138},{t:this.shape_137},{t:this.shape_136},{t:this.shape_135},{t:this.shape_134},{t:this.shape_133},{t:this.shape_132},{t:this.shape_131},{t:this.shape_130},{t:this.shape_129},{t:this.shape_128},{t:this.shape_127},{t:this.shape_126},{t:this.shape_125},{t:this.shape_124},{t:this.shape_123},{t:this.shape_122},{t:this.shape_121},{t:this.shape_120},{t:this.shape_119},{t:this.shape_118},{t:this.shape_117},{t:this.shape_116},{t:this.shape_115},{t:this.shape_114},{t:this.shape_113},{t:this.shape_112},{t:this.shape_111},{t:this.shape_110},{t:this.shape_109},{t:this.shape_108},{t:this.shape_107},{t:this.shape_106},{t:this.shape_105},{t:this.shape_104},{t:this.shape_103},{t:this.shape_102},{t:this.shape_101},{t:this.shape_100},{t:this.shape_99},{t:this.shape_98},{t:this.shape_97},{t:this.shape_96},{t:this.shape_95},{t:this.shape_94},{t:this.shape_93},{t:this.shape_92},{t:this.shape_91},{t:this.shape_90},{t:this.shape_89},{t:this.shape_88},{t:this.shape_87},{t:this.shape_86},{t:this.shape_85},{t:this.shape_84},{t:this.shape_83},{t:this.shape_82},{t:this.shape_81},{t:this.shape_80},{t:this.shape_79},{t:this.shape_78},{t:this.shape_77},{t:this.shape_76},{t:this.shape_75},{t:this.shape_74},{t:this.shape_73},{t:this.shape_72},{t:this.shape_71},{t:this.shape_70},{t:this.shape_69},{t:this.shape_68},{t:this.shape_67},{t:this.shape_66},{t:this.shape_65},{t:this.shape_64},{t:this.shape_63},{t:this.shape_62},{t:this.shape_61},{t:this.shape_60},{t:this.shape_59},{t:this.shape_58},{t:this.shape_57},{t:this.shape_56},{t:this.shape_55},{t:this.shape_54},{t:this.shape_53},{t:this.shape_52},{t:this.shape_51},{t:this.shape_50},{t:this.shape_49},{t:this.shape_48},{t:this.shape_47},{t:this.shape_46},{t:this.shape_45},{t:this.shape_44},{t:this.shape_43},{t:this.shape_42},{t:this.shape_41},{t:this.shape_40},{t:this.shape_39},{t:this.shape_38},{t:this.shape_37},{t:this.shape_36},{t:this.shape_35},{t:this.shape_34},{t:this.shape_33},{t:this.shape_32},{t:this.shape_31},{t:this.shape_30},{t:this.shape_29},{t:this.shape_28},{t:this.shape_27},{t:this.shape_26},{t:this.shape_25},{t:this.shape_24},{t:this.shape_23},{t:this.shape_22},{t:this.shape_21},{t:this.shape_20},{t:this.shape_19},{t:this.shape_18},{t:this.shape_17},{t:this.shape_16},{t:this.shape_15},{t:this.shape_14},{t:this.shape_13},{t:this.shape_12},{t:this.shape_11},{t:this.shape_10},{t:this.shape_9},{t:this.shape_8},{t:this.shape_7},{t:this.shape_6},{t:this.shape_5},{t:this.shape_4},{t:this.shape_3},{t:this.shape_2},{t:this.shape_1},{t:this.shape}]}).wait(1));

	// ShadowShape
	this.instance = new lib.Shape_Drop_Shadow();
	this.instance.parent = this;
	this.instance.alpha = 0.3984;
	this.instance.filters = [new cjs.BlurFilter(8, 8, 1)];
	this.instance.cache(-68,-92,136,184);

	this.timeline.addTween(cjs.Tween.get(this.instance).wait(1));

}).prototype = getMCSymbolPrototype(lib.Surprise, new cjs.Rectangle(-69.7,-94,142,192), null);


// stage content:
(lib.SurpriseGirlFlower = function(mode,startPosition,loop) {
	this.initialize(mode,startPosition,loop,{});

	// timeline functions:
	this.frame_0 = function() {
		this.creditsMC.creditValueField_txt.text = this.properties.credits;
		this.creditsMC.creditLangField_txt.text = this.properties.label;
	}
	this.frame_129 = function() {
		this.properties.callback();
	}

	// actions tween:
	this.timeline.addTween(cjs.Tween.get(this).call(this.frame_0).wait(129).call(this.frame_129).wait(1));

	// Text
	this.creditsMC = new lib.CreditValueContainer();
	this.creditsMC.name = "creditsMC";
	this.creditsMC.parent = this;
	this.creditsMC.setTransform(90,90.05,1.1,1.1);
	this.creditsMC.alpha = 0;

	this.timeline.addTween(cjs.Tween.get(this.creditsMC).wait(114).to({alpha:1},0).to({scaleX:1.65,scaleY:1.65,y:89.95},10).to({scaleX:2.5,scaleY:2.5,y:90.05,alpha:0},5).wait(1));

	// Surprise
	this.instance = new lib.Surprise();
	this.instance.parent = this;
	this.instance.setTransform(90,90.05,0.0999,0.0999,0,0,0,0,0.5);
	this.instance.alpha = 0;

	this.timeline.addTween(cjs.Tween.get(this.instance).to({regY:0,scaleX:1,scaleY:1.0003,y:90,alpha:1},9).to({scaleX:0.8,scaleY:0.8002},10).to({scaleX:1,scaleY:1.0003},5).to({scaleX:0.8,scaleY:0.8002},10).to({scaleX:1,scaleY:1.0003},5).to({scaleX:0.8,scaleY:0.8002},11).to({regY:0.1,scaleX:1,scaleY:1.0003,y:90.1},50).to({regY:0,scaleX:0.0999,scaleY:0.0999,y:90,alpha:0},13).to({_off:true},1).wait(16));

}).prototype = p = new cjs.MovieClip();
p.nominalBounds = new cjs.Rectangle(22.7,70.8,228,117.2);
// library properties:
lib.properties = {
	id: 'E1198412B99C4E3E8EC487C516D2CC35',
	width: 180,
	height: 180,
	fps: 25,
	color: "#FFFFFF",
	opacity: 1.00,
	manifest: [],
	preloads: []
};



// bootstrap callback support:

(lib.Stage = function(canvas) {
	createjs.Stage.call(this, canvas);
}).prototype = p = new createjs.Stage();

p.setAutoPlay = function(autoPlay) {
	this.tickEnabled = autoPlay;
}
p.play = function() { this.tickEnabled = true; this.getChildAt(0).gotoAndPlay(this.getTimelinePosition()) }
p.stop = function(ms) { if(ms) this.seek(ms); this.tickEnabled = false; }
p.seek = function(ms) { this.tickEnabled = true; this.getChildAt(0).gotoAndStop(lib.properties.fps * ms / 1000); }
p.getDuration = function() { return this.getChildAt(0).totalFrames / lib.properties.fps * 1000; }

p.getTimelinePosition = function() { return this.getChildAt(0).currentFrame / lib.properties.fps * 1000; }

an.bootcompsLoaded = an.bootcompsLoaded || [];
if(!an.bootstrapListeners) {
	an.bootstrapListeners=[];
}

an.bootstrapCallback=function(fnCallback) {
	an.bootstrapListeners.push(fnCallback);
	if(an.bootcompsLoaded.length > 0) {
		for(var i=0; i<an.bootcompsLoaded.length; ++i) {
			fnCallback(an.bootcompsLoaded[i]);
		}
	}
};

an.compositions = an.compositions || {};
an.compositions['E1198412B99C4E3E8EC487C516D2CC35'] = {
	getStage: function() { return exportRoot.getStage(); },
	getLibrary: function() { return lib; },
	getSpriteSheet: function() { return ss; },
	getImages: function() { return img; }
};

an.compositionLoaded = function(id) {
	an.bootcompsLoaded.push(id);
	for(var j=0; j<an.bootstrapListeners.length; j++) {
		an.bootstrapListeners[j](id);
	}
}

an.getComposition = function(id) {
	return an.compositions[id];
}



})(createjs = createjs||{}, AdobeAn = AdobeAn||{});
var createjs, AdobeAn;

	var comp = AdobeAn.getComposition('E1198412B99C4E3E8EC487C516D2CC35');
	var lib = comp.getLibrary();
	var ss = comp.getSpriteSheet();
	var images = comp.getImages();
	var ratio = 1;
	// var ratio = (window.devicePixelRatio || 1); //For retina screens

	var surprise = {
		name: 'SurpriseGirlFlower.js',
		lib: lib,
		createjs: createjs,
		canvas: null,
		exportRoot: null,
		stage: null,
		create: function (element, price, label, callback) {
			this.canvas = element;
			this.stage = new lib.Stage(this.canvas);

			AdobeAn.compositionLoaded(lib.properties.id);

			this.exportRoot = new lib['SurpriseGirlFlower']();

			createjs.Ticker.setFPS(lib.properties.fps);
			this.exportRoot.properties = {
				credits: price,
				label: label,
				callback: callback
			};
			this.canvas.width = lib.properties.width * ratio;
			this.canvas.height = lib.properties.height * ratio;
			// this.stage.scaleX = this.ratio; //For retina screens
			// this.stage.scaleY = this.ratio; //For retina screens
			this.stage.update();
			this.stage.addChild(this.exportRoot);
			createjs.Ticker.addEventListener("tick", this.stage);
			this.stage.addChild(this.exportRoot);
		},
		destroy: function() {
			this.stage.removeChild(this.exportRoot);
			createjs.Ticker.removeEventListener("tick", this.stage);
		},
	};

	var ua = window.navigator.userAgent;
	var isInternetExplorer = ua.indexOf('Trident/') > -1;

	if(isInternetExplorer) {
		var event = window.document.createEvent( 'CustomEvent' );
		event.initCustomEvent('SurpriseAnimationModule.onScriptInit', false, false, surprise);
	} else {
		var event = new CustomEvent('SurpriseAnimationModule.onScriptInit', {detail: surprise});
	}

	try {
		var client = window.document.getElementsByClassName('js_member_client')[0];

		if (client) {
			client.dispatchEvent(event);
		}

	} catch (error) {
		console.log('Failed to initailize surprise animation.');
	}
})(window, createjs);