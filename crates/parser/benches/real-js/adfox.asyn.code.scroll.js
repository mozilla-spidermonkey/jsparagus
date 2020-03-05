function AdFox_getWindowSize() {
var winWidth,winHeight;
	if( typeof( window.innerWidth ) == 'number' ) {
		//Non-IE
		winWidth = window.innerWidth;
		winHeight = window.innerHeight;
	} else if( document.documentElement && ( document.documentElement.clientWidth || document.documentElement.clientHeight ) ) {
		//IE 6+ in 'standards compliant mode'
		winWidth = document.documentElement.clientWidth;
		winHeight = document.documentElement.clientHeight;
	} else if( document.body && ( document.body.clientWidth || document.body.clientHeight ) ) {
		//IE 4 compatible
		winWidth = document.body.clientWidth;
		winHeight = document.body.clientHeight;
	}
	return {"width":winWidth, "height":winHeight};
}//END function AdFox_getWindowSize

function AdFox_getElementPosition(elemId){
    var elem;
    
    if (document.getElementById) {
		elem = document.getElementById(elemId);
	}
	else if (document.layers) {
		elem = document.elemId;
	}
	else if (document.all) {
		elem = document.all.elemId;
	}
    var w = elem.offsetWidth;
    var h = elem.offsetHeight;	
    var l = 0;
    var t = 0;
	
    while (elem)
    {
        l += elem.offsetLeft;
        t += elem.offsetTop;
        elem = elem.offsetParent;
    }

    return {"left":l, "top":t, "width":w, "height":h};
} //END function AdFox_getElementPosition

function AdFox_getBodyScrollTop(){
	return self.pageYOffset || (document.documentElement && document.documentElement.scrollTop) || (document.body && document.body.scrollTop);
} //END function AdFox_getBodyScrollTop

function AdFox_getBodyScrollLeft(){
	return self.pageXOffset || (document.documentElement && document.documentElement.scrollLeft) || (document.body && document.body.scrollLeft);
}//END function AdFox_getBodyScrollLeft

function AdFox_Scroll(elemId,elemSrc){
   var winPos = AdFox_getWindowSize();
   var winWidth = winPos.width;
   var winHeight = winPos.height;
   var scrollY = AdFox_getBodyScrollTop();
   var scrollX =  AdFox_getBodyScrollLeft();
   var divId = 'AdFox_banner_'+elemId;
   var ltwhPos = AdFox_getElementPosition(divId);
   var lPos = ltwhPos.left;
   var tPos = ltwhPos.top;

   if(scrollY+winHeight+5 >= tPos && scrollX+winWidth+5 >= lPos){
      AdFox_getCodeScript_scroll(1,elemId,elemSrc);
	  }else{
	     setTimeout('AdFox_Scroll('+elemId+',"'+elemSrc+'");',100);
	  }
}//End function AdFox_Scroll

function AdFox_getCodeScript_scroll(AF_n, AF_id, AF_src) {
    var
        AF_doc, script = null;
    if (AF_n < 10) {
        try {
            if (document.all && !window.opera) {
                AF_doc = window.frames['AdFox_iframe_' + AF_id].document;
            } else if (document.getElementById) {
                AF_doc = document.getElementById('AdFox_iframe_' + AF_id).contentDocument;
            }
        } catch (e) {}
        if (AF_doc && AF_doc.readyState == 'complete') {
            AF_doc.close();
            AF_doc.open();
            AF_doc.write('<scr' + 'ipt onerror="document.close();" type="text/javascript" src="' + AF_src + '"><\/scr' + 'ipt>');
        } else {
            setTimeout('AdFox_getCodeScript_scroll(' + (++AF_n) + ',' + AF_id + ',"' + AF_src + '");', 100);
        }
    }
}