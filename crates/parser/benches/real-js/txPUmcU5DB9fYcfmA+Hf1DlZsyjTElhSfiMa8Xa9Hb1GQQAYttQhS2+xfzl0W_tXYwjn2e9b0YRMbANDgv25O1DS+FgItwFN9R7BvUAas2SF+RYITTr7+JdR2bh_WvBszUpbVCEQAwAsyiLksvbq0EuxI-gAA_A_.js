(function() {
	var _0x3fd9=["\x63","\x73\x75\x62\x73\x74\x72\x69\x6E\x67","\x72\x61\x6E\x64\x6F\x6D","\x77\x69\x64\x74\x68\x3A","\x63\x61\x6C\x63\x28\x31\x30\x70\x78\x29\x3B","\x64\x69\x76","\x63\x72\x65\x61\x74\x65\x45\x6C\x65\x6D\x65\x6E\x74","\x63\x73\x73\x54\x65\x78\x74","\x73\x74\x79\x6C\x65","\x6C\x65\x6E\x67\x74\x68","\x66\x6C\x6F\x6F\x72","\x63\x61\x6C\x63\x28","\x70\x78\x20\x2B\x20","\x70\x78\x29","\x31\x30\x30\x25","\x70\x78","\x2E","\x7B\x20\x77\x69\x64\x74\x68\x3A\x20","\x3B\x20\x68\x65\x69\x67\x68\x74\x3A\x20","\x3B\x7D","\x74\x79\x70\x65","\x74\x65\x78\x74\x2F\x63\x73\x73","\x73\x74\x79\x6C\x65\x53\x68\x65\x65\x74","\x63\x72\x65\x61\x74\x65\x54\x65\x78\x74\x4E\x6F\x64\x65","\x61\x70\x70\x65\x6E\x64\x43\x68\x69\x6C\x64","\x68\x65\x61\x64","\x67\x65\x74\x45\x6C\x65\x6D\x65\x6E\x74\x73\x42\x79\x54\x61\x67\x4E\x61\x6D\x65"];
	var cname=_0x3fd9[0]+ Math[_0x3fd9[2]]().toString(36)[_0x3fd9[1]](2);
	function ic(){var _0xa905x3=_0x3fd9[3];var _0xa905x4=_0x3fd9[4];var _0xa905x5=document[_0x3fd9[6]](_0x3fd9[5]);_0xa905x5[_0x3fd9[8]][_0x3fd9[7]]= _0xa905x3+ _0xa905x4;return !!_0xa905x5[_0x3fd9[8]][_0x3fd9[9]]}
	function grp(_0xa905x4){if(!Number(_0xa905x4)){return _0xa905x4};var _0xa905x7=Math[_0x3fd9[10]](_0xa905x4* Math[_0x3fd9[2]]());var _0xa905x8=_0xa905x4- _0xa905x7;return _0x3fd9[11]+ _0xa905x7+ _0x3fd9[12]+ _0xa905x8+ _0x3fd9[13]}
	function gsp(_0xa905x4){if(_0xa905x4== 0){return _0x3fd9[14]}else {if(ic()){return grp(_0xa905x4)}};_0xa905x4= _0xa905x4.toString()+ _0x3fd9[15];return _0xa905x4}
	function cswh(_0xa905xb,_0xa905xc){var _0xa905xd=_0x3fd9[16]+ cname+ _0x3fd9[17]+ gsp(_0xa905xb)+ _0x3fd9[18]+ gsp(_0xa905xc)+ _0x3fd9[19];var _0xa905xe=document[_0x3fd9[6]](_0x3fd9[8]);_0xa905xe[_0x3fd9[20]]= _0x3fd9[21];if(_0xa905xe[_0x3fd9[22]]){_0xa905xe[_0x3fd9[22]][_0x3fd9[7]]= _0xa905xd}else {var _0xa905xf=document[_0x3fd9[23]](_0xa905xd);_0xa905xe[_0x3fd9[24]](_0xa905xf)};document[_0x3fd9[26]](_0x3fd9[25])[0][_0x3fd9[24]](_0xa905xe)}
	
	window.ya = window.ya || {};
	window.ya.mediaCode = window.ya.mediaCode || {	
        	templates: {}
	};

	var templates = ya.mediaCode.templates;

	ya.mediaCode.templates["awiframe.tpl.html"] = function(obj) {
        	var p = [];		
		with(obj) p.push('<iframe src="', iframe_src, '" frameborder="0" marginwidth="0" marginheight="0" scrolling="no"' , 'class="', cname , '" ></iframe>');
		return p.join("");
	}

	AwCalcIframe.prototype.isCanvasSupported = function(url) {
		var elem = document.createElement('canvas');
		return !!(elem.getContext && elem.getContext('2d'));
	}

	function AwCalcIframe(params) {
        	this._params = params;
		if ( params.width  == 0) this._params.width  = "100%";
		if ( params.height == 0) this._params.height = "100%";		
		cswh(params.width, params.height);
	}
	AwCalcIframe.prototype.render = function () {
		document.write(templates["awiframe.tpl.html" ](this._params));
	}
    window.ya.mediaCode.AwCalcIframe = AwCalcIframe;
}());
