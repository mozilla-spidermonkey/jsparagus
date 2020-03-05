if (typeof ncd == 'undefined') ncd = {};

/* Global variable */
if (typeof ncd.ccsrv == 'undefined') { ncd.ccsrv = "cc.naver.com";}	// server domian
if (typeof ncd.nclkModule == 'undefined') { ncd.nclkModule = "cc";}	// module name
// Search Arguments
if (typeof ncd.g_pid == 'undefined') { ncd.g_pid = "";}
if (typeof ncd.g_sid == 'undefined') { ncd.g_sid = "";}
// temporary image object

var nclkImg = [];
ncd.version = "1.2.11D";
ncd.nclkDetect = navigator.userAgent.toLowerCase();
// Function Name : getScrollBarWidth
// Return : scrollbar width 
// Description : Get scrollbar width in order to calculate browser size of Opera.
ncd.getScrollBarWidth = function() {
	var inner = document.createElement('p');
	inner.style.width = '200px';
	inner.style.height = '200px';

	var outer = document.createElement('div');
	outer.style.position = 'absolute';
	outer.style.top = '0px';
	outer.style.left = '0px';
	outer.style.visibility = 'hidden';
	outer.style.width = '200px';
	outer.style.height = '150px';
	outer.style.overflow = 'hidden';
	outer.appendChild (inner);

	document.body.appendChild (outer);
	var w1 = inner.offsetWidth;
	outer.style.overflow = 'scroll';
	var w2 = inner.offsetWidth;

	if (w1 == w2) w2 = outer.clientWidth;

	document.body.removeChild (outer);

	return (w1 - w2);
};

// Function Name : findPos
// Parameter : obj - owner of event occurence object
// Return : object position
// Description : Get offsetLeft, offsetTop
ncd.findPos = function(obj) {
	var curleft = curtop = 0;

	try {
		if (obj.offsetParent) {
			do {
				curleft += obj.offsetLeft;
				curtop += obj.offsetTop;
			} while (obj = obj.offsetParent);
		}
		else if (obj.x || obj.y)			// for safari of old version
		{
			if (obj.x) curleft += obj.x;
			if (obj.y) curtop += obj.y;
		}
	} catch(e) {
	}
	return [curleft, curtop];
};

// Function Name : windowSize
// Paramenter : win -  window object
// Return : window width
// Description : Calculates inner width of browser window.
ncd.windowSize = function(win) {
	if (!win) win = window;
	var winWidth = 0;
	if (win.innerWidth) {
		// most non-IE
		winWidth = win.innerWidth;
		// including scroll bar !! 
		if ( typeof(win.innerWidth) == 'number') {
			var scrollbarWidth = ncd.getScrollBarWidth();
			winWidth = win.innerWidth - scrollbarWidth; // Opera includes scrollbar width at innerWidth
		}
	} else if (document.documentElement && document.documentElement.clientWidth) {
		//IE 6+ in 'standards compliant mode'
		winWidth = document.documentElement.clientWidth; 
	} else if( document.body && ( document.body.clientWidth || document.body.clientHeight ) ) {
		//IE 4 compatible
		winWidth = document.body.clientWidth; 
	}
	return winWidth; 
};

// Function Name : ncd.checkIframe
// Parameter : obj - owner of event occurence object
// Return Value : iframe ID or false
// Description : Check whether it is clicked in iframe or not
ncd.checkIframe = function(obj) {
	var oriURL = document.URL;
	var p = obj.parentNode;
	var docObj;
	var ifrId;
	if (p == null || p == undefined) return false;
	while (1) {
		if (p.nodeName.toLowerCase() == "#document") {
			if (p.parentWindow) {		// IE, Opera
				docObj = p.parentWindow;
			} else {	// Firefox, Safari
				docObj = p.defaultView;
			}
			try {
				if (docObj.frameElement != null && docObj.frameElement != undefined ) {	
					if (docObj.frameElement.nodeName.toLowerCase() == "iframe") {
						ifrId = docObj.frameElement.id;	// Get iframe id
						if (!ifrId) return false;
						return ifrId;					
					} else {
						return false;
					}
				} else {
					return false;
				}
			} catch (e) {
				return false;
			}
		} else {
			p = p.parentNode;
			if (p == null || p == undefined) return false;
		}
	}
};

// Function Name : absPath
// Parameter : url - relative URL
// Return Value : absolute URL
// Description : convert relative URL to absolute URL
ncd.absPath = function (url){
	var wl = window.location;
	var loc = wl.href;
	var mainUrl = wl.protocol + "//" + wl.host;

	if (url.charAt(0) == '/') {
		if (url.charAt(1) == '/') {
			return wl.protocol + url;
		} else {
			return mainUrl + url;
		}
	}

	if (/^\.\//.test(url)) {
		url = url.substring(2);
	}

	while (/^\.\./.test(url)) {
		if (mainUrl != loc) {
			loc = loc.substring(0, loc.lastIndexOf('/'));
		}
		url = url.substring(3);
	}

	if (mainUrl != loc) {
		if (url.charAt(0) != '?' && url.charAt(0) != '#') {
			loc = loc.substring(0, loc.lastIndexOf('/'));
		}
	}

	if (url.charAt(0) == '/') {
		return loc + url;
	} else if(url.charAt(0) == '?') {
		loc = loc.split('?')[0];
		return loc + url;
	} else if(url.charAt(0) == '#') {
		loc = loc.split('#')[0];
		return loc + url;
	} else {
		return loc + "/" + url;
	}
};

// Function Name : getPosition
// Argument : theEvent, ifrId
// Description : decide mode
ncd.getPosition = function (theEvent, ifrId)
{
	var px=-1, py=-1, sx=-1, sy=-1;
	var dBody = document.body;
	var dElement = document.documentElement;
	
	if (ifrId) {	// click object in iframe
		var ifrOffset = ncd.findPos(document.getElementById(ifrId));	// Get position of iframe
		var ifrSx=ifrSy=-1;
		if (theEvent.clientX  && theEvent.clientX != undefined) {
			if (dBody.clientLeft && dBody.clientTop) {
				ifrSx = theEvent.clientX - dBody.clientLeft;		//  relative position in iframe
				ifrSy = theEvent.clientY - dBody.clientTop;
			} else { // firefox bug - clientLeft, clientTop don't work in firefox. It is OK in firefox 3.0
				ifrSx = theEvent.clientX;
				ifrSy = theEvent.clientY;
			}
		}
		// postion of iframe + relative position in iframe
		px = ifrOffset[0] + ifrSx;
		py = ifrOffset[1] + ifrSy;

		// relative position in window
		if (dBody && (dBody.scrollTop || dBody.scrollLeft)) {
			sx = px - dBody.scrollLeft;
			sy = py - dBody.scrollTop;	
		} else if (dElement && (dElement.scrollTop || dElement.scrollLeft)) {
			sx = px - dElement.scrollLeft;
			sy = py - dElement.scrollTop;
		} else {
			sx = px;
			sy = py;
		}
	} else {	// normal click object
		if (theEvent.clientX  && theEvent.clientX != undefined) {
			if (dBody.clientLeft && dBody.clientTop) {
				sx = theEvent.clientX - dBody.clientLeft;		// relative position in window
				sy = theEvent.clientY - dBody.clientTop;
			} else { // firefox bug - clientLeft, clientTop don't work in firefox. It is OK in firefox 3.0
				sx = theEvent.clientX;
				sy = theEvent.clientY;
			}
		}

		// IE calculates (clientX,clientY) relativ to window, not content.
		if (dBody && (dBody.scrollTop || dBody.scrollLeft)) {
			px = dBody.scrollLeft + (sx < 0 ? 0: sx);	
			py = dBody.scrollTop + (sy < 0 ? 0: sy);				
		} else if (dElement && (dElement.scrollTop || dElement.scrollLeft)) {
			if (dElement.scrollLeft !=undefined) px = dElement.scrollLeft + (sx < 0 ? 0: sx);	
			if (dElement.scrollTop !=undefined) py = dElement.scrollTop + (sy < 0 ? 0: sy);			
		} else {
			px = (sx < 0 ? 0: sx);
			py = (sy < 0 ? 0: sy);
		}
		
		if (theEvent.pageX) { px = theEvent.pageX; }
		if (theEvent.pageY) { py = theEvent.pageY; }
	}
	return [px, py, sx, sy]
};

// Function Name : decideMode
// Argument : obj, opt
// Description : decide mode
ncd.decideMode = function (obj, opt)
{
	var mode = 0;
	var tN, tH;
	var ccsrv = ncd.ccsrv;
	var nclkIsMSIE = /msie/.test( ncd.nclkDetect ) && !/opera/.test( ncd.nclkDetect );
	var srvDomain = ccsrv.substring(ccsrv.indexOf(".")+1);
	// Case of no redirection
	// 1. Open link to new window or target of other window or object of its window ( ex. iframe) 
	// 2. Use javascript scheme (javascript:)
	// 3. Use fragment at the first character of href (ex. #top)
	// 4. No use href
	// 5. opt is 1.
	// 6. Send to different domain in IE6, IE7
	if (opt.indexOf("1") != -1) {
		mode = 0;
	} else if (obj.href) {
		tN = obj.nodeName.toLowerCase();
		tH = obj.href.toLowerCase();
		if ((obj.target && obj.target != "_self" && obj.target != "_top" && obj.target != "_parent") 
			|| (tH.indexOf("javascript:") != -1) 
			|| (obj.getAttribute("href",2) && obj.getAttribute("href",2).charAt(0) == '#')	// If the browser is IE8, obj.getAttribute("href") is null at <img>. 
			|| (tH.indexOf("#") != -1 && (tH.substr(0,tH.indexOf("#")) == document.URL))
			|| tN.toLowerCase() == "img" 
			|| nclkIsMSIE && window.location.host.indexOf(srvDomain) == -1) {
			mode = 0;
		} else {	// Case of redirection - Open link to its window (no target, _self)
			mode = 1;
		}
	} else {
		mode = 0;
	}
	return mode;
};

// Function Name : makeURL
// Argument : nsc, obj, area, cid, rank, opt, sign, mode, bw, px, py, sx, sy
// Description : make request URL
ncd.makeURL = function (nsc, obj, area, cid, rank, opt, sign, mode, bw, px, py, sx, sy)
{
	var serviceType = 0;	// normal log or search log
	var clink = "";
	var ptc = (window.location.protocol=="https:")?"https:":"http:";
	var tN = obj.nodeName.toLowerCase();
	var tH = obj.href.toLowerCase();
	var tY = "";
	var ccsrv = ncd.ccsrv;
	
	if (opt.indexOf('n') == 0) {
		serviceType = 0;	
	} else {
		if (ncd.g_ssc != undefined && ncd.g_query !=undefined) {
			serviceType = 1;
		} else {
			serviceType = 0;
		}
	}

	if (obj.href && obj.href.indexOf(ptc+"//"+ccsrv) == 0) {
		clink = obj.href;
	} else {
		clink = ptc + "//" + ccsrv + "/"+ ncd.nclkModule + "?a=" + area + "&r=" + rank + "&i=" + cid;
		clink += "&bw=" + bw + "&px="+ px + "&py=" + py + "&sx="+ sx + "&sy=" + sy + "&m=" + mode;
		if (serviceType == 0) {
			clink += "&nsc=" + nsc;
		} else if (serviceType == 1) {
			clink += "&ssc=" + ncd.g_ssc + "&q=" + encodeURIComponent(ncd.g_query) + "&s=" + ncd.g_sid + "&p=" + ncd.g_pid + "&g=" + sign;		
		}
		if (tH && tH.indexOf(ptc+"//"+ccsrv) != 0 && tN.toLowerCase() != "img") {
			var h = obj.href;			
			if (obj.outerHTML && !window.XMLHttpRequest) { // detect IE6 or below.
				h = (/\shref=\"([^\"]+)\"/i.test(obj.outerHTML) && RegExp.$1).replace(/\\/g, "\\\\").replace(/%([A-Z0-9]{2})/ig, "\\$1");
				(d = document.createElement("div")).innerHTML = h;
				h = d.innerText.replace(/\\([A-Z0-9]{2})/gi, "%$1").replace(/\\\\/g, "\\");
			}
			tH = h.toLowerCase();
			if (tH.indexOf("http:") == 0 || tH.indexOf("https:") == 0 || tH.indexOf("javascript:") == 0) {
				clink += "&u="+encodeURIComponent(h);
			} else {
				tU = ncd.absPath(h);		// IE bug - Manually change obj.href to relative URL, IE don't change obj.href to absolute URL
				clink += "&u="+encodeURIComponent(tU);
			}
		} else {
			if (obj.href) {
				if(obj.href.length > 0) {	// EX) option=1 & mailto:abc@naver.com
					clink += "&u="+encodeURIComponent(obj.href);
				} else {
					clink += "&u=about%3Ablank";
				}
			} else {
				clink += "&u=about%3Ablank";			
			}
		}
	}

	return clink;
};

// Function Name : sendRequet
// Argument : obj-  owner of event occurence object, mode, clink
// Description : send request to log server
ncd.sendRequet = function (obj, mode, clink) {
	var o, tempStr;
	var nclkIsSafari = (ncd.nclkDetect.indexOf('safari') != -1 ? true : false);
	if (mode == 1)  {	// Redirection
		tempStr = obj.innerHTML;
		obj.href = clink;
		
		if (obj.innerHTML != tempStr) {
			obj.innerHTML = tempStr;
		}
	} else if (document.images)  {	// No redirection
		var timestr = new Date().getTime();
		clink += "&time="+ timestr;		// Aviod image cache

		if (nclkIsSafari && !obj.href) {
			var b = c = new Date();
			while ((b.getTime() - c.getTime()) < 100) b = new Date();
			var o = new Image();
			nclkImg.push(o);		
			o.src = clink;
		} else {
			var o = new Image();
			nclkImg.push(o);
			o.src = clink;
		}
	}
};

// Function Name : clickcrD
// Argument : nsc- nhn service code, obj-  owner of event occurence object, area - click area,  cid - gdID or cid, rank - rank in area, event - predefined event object, opt - forced mode setting, sign - signature of image
// Description : When users click link, this function is called. It request to the server, then the server log data.
ncd.clickcrD = function (nsc, obj, area, cid, rank, evt, opt, sign)
{
	if (area == "" || typeof area == 'undefined' ) {
		return;
	}
	if (!opt) { 
		opt = "0";
	} else {
		opt = String(opt);
	}
	if (!sign) sign = "";	

	var theEvent = window.event ? window.event : evt;
	var px=-1, py=-1, sx=-1, sy=-1;
	var dBody, dElement, ifrId, mode, clink, psArr;
	
	try {
		bw = ncd.windowSize(window);	// size of window
		ifrId = ncd.checkIframe(obj);	// check whether it is clicked in iframe or not
		psArr = ncd.getPosition(theEvent, ifrId)
		px = psArr[0];
		py = psArr[1];
		sx = psArr[2];
		sy = psArr[3];
	} catch (e) {
	}

	mode = ncd.decideMode(obj, opt);

	clink = ncd.makeURL(nsc, obj, area, cid, rank, opt, sign, mode, bw, px, py, sx, sy);
	ncd.sendRequet(obj, mode, clink);

	return true;	
};
// Function Name : clickcr
// Argument : obj-  owner of event occurence object, area - click area,  cid - gdID or cid, rank - rank in area, event - predefined event object, opt - forced mode setting, sign - signature of image
// Description : When users click link, this function is called. It request to the server, then the server log data.
ncd.clickcr = function (obj, area, cid, rank, evt, opt, sign) {
	var nsc = "decide.me";
	if (ncd.nsc) { 
		var nsc = ncd.nsc;
	}
	ncd.clickcrD(nsc, obj, area, cid, rank, evt, opt, sign);
};