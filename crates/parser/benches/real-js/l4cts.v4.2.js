/* Logger(TM) for Ad (Standalone) tracking script Version.4.2 */
/* COPYRIGHT 2002-2015 BIZSPRING INC. */
/* DO NOT MODIFY THIS SCRIPT. */

(function(){
	var _TRK_LAID,_TRK_CTS;
	var _TAD_DOMAIN="ssl.logger.co.kr";
	var _trk_bMSIE=(document.all)?true:false;
	var _trk_bJS12=(window.screen)?true:false;
    var _trk_tD = _tcts_getRootDomain(self.document.location.href);
	var _TAD_CKDOM=((typeof _L_LACD)!="undefined"&&_L_LACD!="")?_L_LACD:"."+_trk_tD;
	if((typeof _L_LALT)!="undefined"){var _TRK_LIFE=_L_LALT;}else{var _TRK_LIFE=30;}
	var _TRK_VISIT_NEW = "";
    var _TD=new Date();

	function _tcts_escape(_str){
		var str="",ch;
		var bEncURI="N";
		try{bEncURI=encodeURI('Y');}catch(_e){ }
		if(bEncURI=="Y")str=encodeURI(_str);
		else str=escape(_str);
		str=str.split("+").join("%2B");
		str=str.split("/").join("%2F");
		str=str.split("&").join("%26");
		str=str.split("?").join("%3F");
		str=str.split(":").join("%3A");
		str=str.split("#").join("%23");
		return str;
	}
	function _tcts_setCookie(name,value,expire) {
		var today=new Date();
		today.setTime(today.getTime()+ expire);

		var domainStr = "";
		if((typeof _TAD_CKDOM)!="undefined" && _TAD_CKDOM!="") domainStr = "domain="+_TAD_CKDOM+";";
		document.cookie=name+"="+value+"; path=/; "+domainStr+" expires="+today.toGMTString()+";";
	}
	function _tcts_getCookie(name) {
		var cookieName=name+"=";
		var x=0;
		while(x<=document.cookie.length) {
			var y=(x+cookieName.length);
			if(document.cookie.substring(x,y)==cookieName) {
				if((endOfCookie=document.cookie.indexOf(";",y))==-1) endOfCookie=document.cookie.length;
				return unescape(document.cookie.substring(y,endOfCookie));
			}
			x=document.cookie.indexOf(" ",x)+1;
			if(x == 0) break;
		}
		return "";
	}
	function _tcts_setStorage(name,value,expire,sNum,sDom){
		var today=new Date();
		today.setTime(_TD.getTime()+expire);
		if(sNum!="")name=name+"_"+sNum;
		if((typeof localStorage)!="undefined"&&localStorage){
			var valueL=value+"_||_"+today.getTime();
			localStorage.setItem(name,valueL);
		}
	}
	function _tcts_getStorage(name,sNum){
		var str="";
        if(sNum==null) sNum="";
		if(sNum!="") name=name+"_"+sNum;
		if((typeof localStorage)!="undefined"&&localStorage){
			str=localStorage.getItem(name);
			if(str==null){
				str="";
			}else{
				var oldC=str.split("_||_");
				var cExp=oldC[1];
				var expGap=cExp-new Date().getTime();
				if(expGap>0){
					str=oldC[0];
				}else{
					str="";
					localStorage.removeItem(name);
				}
			}
		}

        if( str=="undefined"||str=="" ) {
            var cookieName=name+"=";
            var x=0;
            var str2 = "";
            while(x<=document.cookie.length){
                var y=(x+cookieName.length);
                if(document.cookie.substring(x,y)==cookieName){
                    if((endOfCookie=document.cookie.indexOf(";",y))==-1) endOfCookie=document.cookie.length;
                    str2 = (document.cookie.substring(y,endOfCookie));
                    break;
                }
                x=document.cookie.indexOf(" ",x)+1;
                if(x==0) break;
            }
            if( str2!="undefined"&&str2!="" ) str = str2;
        } else if( str.indexOf( "_,_" )>0 ) {
            var cookieName=name+"=";
            var x=0;
            var str2 = "";
            while(x<=document.cookie.length){
                var y=(x+cookieName.length);
                if(document.cookie.substring(x,y)==cookieName){
                    if((endOfCookie=document.cookie.indexOf(";",y))==-1) endOfCookie=document.cookie.length;
                    str2 = (document.cookie.substring(y,endOfCookie));
                    break;
                }
                x=document.cookie.indexOf(" ",x)+1;
                if(x==0) break;
            }
            if( str2.indexOf( "_,_" )>0 ) {
                var strC=new Array();
                var today=new Date();

                var strC=str.split("_|_");
                var strD=str2.split("_|_");
                for(var i=0;i<strD.length;i++){
                    var tmp=strD[i].split("_,_");
                    var ce=tmp[0];
                    var cv=tmp[1];
                    if(ce>today && str.indexOf(cv)<0 ){
                        strC[strC.length]=strD[i];
                    }
                }
                str = strC.join("_|_");
            }
        }
        return str;
	}
	function _tcts_getCCA(CCN,sNum,sDom){
		var str=_tcts_getStorage(CCN,sNum);

		if(str==null){
			return"";
		}else{
			var cExp=0;
			var oldC=str.split("_|_");
			var newC=new Array();
			var newV="";
			var today=new Date();
			for(var i=0;i<oldC.length;i++){
				var tmp=oldC[i].split("_,_");
				var ce=tmp[0];
				var cv=tmp[1];
				if(ce>today){
					newC[newC.length]=oldC[i];
					cExp=cExp>ce?cExp:ce;
				}
			}
			newC.sort();
			newV=newC.join("_|_");
			var expGap=cExp-new Date().getTime();
			if(expGap>0){
                _tcts_setStorage(CCN,newV,expGap,sNum,sDom);
			}
			return newV;
		}
	}
	function _tcts_setCCA(CCN,value,expire,sNum,sDom){
		var str=_tcts_getCCA(CCN,sNum,sDom);
        if( CCN.indexOf("TRK_ACQ")>0 && str.indexOf(value)>0 ) {
            return false;
        } else {
            if(str==null){
                str="";
            } else {
                var cExp=0;
                var oldC=str.split("_|_");
                var newC=new Array();
                var newV="";
                var today=new Date();
                for(var i=0;i<oldC.length;i++){
                    var tmp=oldC[i].split("_,_");
                    var ce=tmp[0];
                    var cv=tmp[1];
                    if(ce>today){
                        newC[newC.length]=oldC[i];
                        cExp=cExp>ce?cExp:ce;
                    }
                }
            }
            today.setTime(_TD.getTime()+expire);
            var ce=today.getTime();
            cExp=cExp>ce?cExp:ce;
            newC[newC.length]=ce+"_,_"+value;

            newC.sort();
            newV=newC.join("_|_");
            while(newV.length>1200){
				newC = newC.slice(1);
				newV = newC.join("_|_");
            }
            var expGap=cExp-new Date().getTime();
            if(expGap>0){
                _tcts_setStorage(CCN,newV,expGap,sNum,sDom);
            }
            return true;
        }
	}
	function _tcts_getParameter(name){
		var paraName=name+"=";
		var URL=""+self.document.location.search;
		var tURL="";
		try{ tURL=top.document.location.search; }catch(_e){}
		URL=URL+"&"+tURL;
		if(URL.indexOf(paraName)!=-1){
			var x=URL.indexOf(paraName)+paraName.length;
			var y=URL.substr(x).indexOf("&");
			if(y!=-1)return URL.substring(x,x+y);
			else return URL.substr(x);
		}
		return""
	}
	function _tcts_getNewSID(len){
		var str="01234567890abcdef";
		var ret="";
		for(var i=0;i<len;i++){
			ret=ret+(str.substr(Math.floor(Math.random()*str.length),1))
		}
		return ret
	}
    function _tcts_getRootDomain(urlStr) {
        var CDs  = new Array("ac","ad","ae","af","ag","ai","al","am","ao","aq","ar","as","at","au","aw","ax","az","ba","bb","bd","be","bf","bg","bh","bi","bj","bm","bn","bo","br","bs","bt","bw","by","bz","ca","cc","cd","cf","cg","ch","ci","ck","cl","cm","cn","co","cr","cu","cv","cw","cx","cy","cz","de","dj","dk","dm","do","dz","ec","ee","eg","er","es","et","eu","fi","fj","fk","fm","fo","fr","ga","gd","ge","gf","gg","gh","gi","gl","gm","gn","gp","gq","gr","gs","gt","gu","gw","gy","hk","hm","hn","hr","ht","hu","id","ie","il","im","in","io","iq","ir","is","it","je","jm","jo","jp","ke","kg","kh","ki","km","kn","kp","kr","kw","ky","kz","la","lb","lc","li","lk","lr","ls","lt","lu","lv","ly","ma","mc","md","me","mg","mh","mk","ml","mm","mn","mo","mp","mq","mr","ms","mt","mu","mv","mw","mx","my","mz","na","nc","ne","nf","ng","ni","nl","no","np","nr","nu","nz","om","pa","pe","pf","pg","ph","pk","pl","pm","pn","pr","ps","pt","pw","py","qa","re","ro","rs","ru","rw","sa","sb","sc","sd","se","sg","sh","si","sk","sl","sm","sn","so","sr","ss","st","su","sv","sx","sy","sz","tc","td","tf","tg","th","tj","tk","tl","tm","tn","to","tr","tt","tv","tw","tz","ua","ug","uk","us","uy","uz","va","vc","ve","vg","vi","vn","vu","wf","ws","ye","yt","za","zm","zw");
        var NCDs = new Array("aero","an","arpa","asia","bike","biz","bv","camera","cat","clothing","com","construction","contractors","coop","diamonds","directory","edu","enterprises","equipment","estate","gallery","gb","gov","graphics","guru","holdings","info","int","jobs","kitchen","land","lighting","menu","mil","mobi","museum","name","net","org","photography","plumbing","post","pro","sexy","singles","sj","tattoo","technology","tel","tips","today","tp","travel","uno","ventures","voyage","xxx");

        var tmp = urlStr;
        tmp = tmp.replace(/http(s){0,1}:\/\//gi, '');
        tmp = tmp.replace(/\/.*/gi, '');
        tmp = tmp.replace(/:[0-9]+/gi, '');

        var domain = tmp.toLowerCase();
        if(domain.match(/^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}$/)) {
            return domain;
        } else {
            var de = domain.split(".");
            var TLD = de[de.length-1];

            if(_tcts_indexOf(CDs, TLD) != -1 || _tcts_indexOf(NCDs, TLD) != -1) {
                if(_tcts_indexOf(CDs, TLD) != -1) {
                    var krSecondDomains = new Array("co","ne","or","re","pe","go","mil","ac","hs","ms","es","sc","kg","seoul","busan","daegu","incheon","gwangju","daejeon","ulsan","gyeonggi","gangwon","chungbuk","chungnam","jeonbuk","jeonnam","gyeongbuk","gyeongnam","jeju");
                    if(TLD == "kr") {
                        if(_tcts_indexOf(krSecondDomains, de[de.length-2]) != -1) {
                            if(de[de.length-3] != null && de[de.length-3] != "") {
                                return de[de.length-3]+"."+de[de.length-2]+"."+de[de.length-1];
                            } else {
                                return de[de.length-2]+"."+de[de.length-1];
                            }
                        } else {
                            return de[de.length-2]+"."+de[de.length-1];
                        }
                    } else {
                        return domain;
                    }
                } else if(_tcts_indexOf(NCDs, TLD) != -1) {
                    if(de[de.length-2] != null && de[de.length-2] != "") {
                        return de[de.length-2] + "." + de[de.length-1];
                    } else {
                        return de[de.length-2];
                    }
                } else {
                    return domain;
                }
            } else {
                return domain;
            }
        }
    }
    function _tcts_indexOf(arr, obj) {
        for (var i=0; i< arr.length; i++) {
            if (arr[i] === obj) {
                return i;
            }
        }
        return -1;
    }
    function _tcts_shortenURL(sourceURL) {
        function detectAndCut(paramName, paramValues) {
            var returnArray = new Array();
            for(var j=0; j<paramValues.length; j++) {
                var paramValuePair = paramValues[j].split("=");
                var param, value;
                if(paramValuePair.length<=2) {
                    param = paramValuePair[0];
                    value = paramValuePair[1];
                } else {
                    param = paramValuePair[0];
                    for(var p=1; p<paramValuePair.length; p++) {
                        value += paramValuePair[p];
                    }
                }

                if(param == paramName) {
                    returnArray.push(param + "=");
                } else {
                    returnArray.push(paramValues[j]);
                }
            }
            return returnArray;
        }
        var domainParamDetect = new Array("cr2.shopping.naver.com|x");
        var paramParamDetect = new Array("Ncisy|NaPm", "Ncisy|Ncisy");
        var newURL = "";
        var domainQueries = sourceURL.split("?");
        var paramValues = new Array();

        if(domainQueries.length == 2) {
            var domain = domainQueries[0].toLowerCase();
            var query = domainQueries[1];
            paramValues = query.split("&");

            if(domain != "" && paramValues.length>0) {
                for(var i=0; i<domainParamDetect.length; i++) {
                    var detectCutPair = domainParamDetect[i].split("|");
                    var detect = detectCutPair[0].toLowerCase();
                    var cut = detectCutPair[1];

                    if(domain.indexOf(detect)>=0) {
                        paramValues = detectAndCut(cut, paramValues);
                    }
                }
            }

            if(query != "" && paramValues.length>0) {
                for(var i=0; i<paramParamDetect.length; i++) {
                    var detectCutPair = paramParamDetect[i].split("|");
                    var detect = detectCutPair[0];
                    var cut = detectCutPair[1];

                    if(query.indexOf(detect)>=0) {
                        paramValues = detectAndCut(cut, paramValues);
                    }
                }
            }
            newURL = domain + (paramValues.length > 0 ? ("?" + paramValues.join("&")) : "");
        } else if(domainQueries.length == 1) {
            paramValues = domainQueries[0].split("&");
            if(paramValues.length>0) {
                for(var i=0; i<paramParamDetect.length; i++) {
                    var detectCutPair = paramParamDetect[i].split("|");
                    var detect = detectCutPair[0];
                    var cut = detectCutPair[1];

                    if(domainQueries[0].indexOf(detect)>=0) {
                        paramValues = detectAndCut(cut, paramValues);
                    }
                }
            }
            newURL = paramValues.join("&");
        } else {
            newURL = sourceURL;
        }
        return newURL;
    }

    function _trk_escape(_str) {
		var str, ch;
		var bEncURI = "N"; try{bEncURI=encodeURI('Y');}catch(_e){}
		if( bEncURI == "Y" ) str=encodeURI(_str); else str = escape(_str);
		str=str.split("+").join("%2B");
		str=str.split("/").join("%2F");
		str=str.split("&").join("%26");
		str=str.split("?").join("%3F");
		str=str.split(":").join("%3A");
		str=str.split("#").join("%23");
		return str;
	}

	function _tcts_make_code(_TRK_AU){
        var t = new Date;
        var tye=(_trk_bMSIE)?(t.getYear()):(t.getFullYear()); var tmo=t.getMonth()+1; var tda=t.getDate();
        var tho=t.getHours(); var tmi=t.getMinutes(); var tse=t.getSeconds();
        var tzo=t.getTimezoneOffset();

		var dr=self.document.referrer;
		var tdr="";
		try{ tdr=top.document.referrer; }catch(_e){}
		var tdu="";
		try{ tdu=top.document.location.href; }catch(_e){}
		var bFrm=false;
		if(dr==tdu){
			dr=tdr;
			bFrm=true;
		}
		if(dr=="undefined") dr="";
		var du=self.document.location.href;
		if(du.substr(0,4)=="file") return"";
		var adKeyVal="";
		if(bFrm){
			var adParams=new Array("BSPRG","BSCPN","BSCCN1","BSCCN2","BSCCN3","BSCCN4","BSCCN5","BSCCN6","BSCCN7","BSCCN8","BSCCN9","BSCCN10");
			for(var i=0;i<adParams.length;i++){
				adKeyVal=_tcts_getParameter(adParams[i]);
				if(adKeyVal!=""&&du.indexOf(adParams[i]+"=")<0) if(du.indexOf("?")!=-1) du=du+"&"+adParams[i]+"="+adKeyVal; else du=du+"?"+adParams[i]+"="+adKeyVal;
			}
		}
		if((typeof _L_LALT)!="undefined") _TRK_LIFE=_L_LALT;
		else _TRK_LIFE=30;
		if(!dr)dr="";
		if(!du)du="";
		var tc="";
		var prtcl=document.location.protocol.indexOf("https")!=-1?"https://":"http://";
		tc=tc+prtcl+_TAD_DOMAIN;
		tc=tc+"/tracker_L4Ad.tsp";
		tc=tc+"?u="+_TRK_AU+"&XU=&TREX="+_TRK_AEX;
		if(_TRK_CTS=="CTS"||_TRK_CTS=="Y")tc=tc+"&BSCTS=Y";
		else if(_TRK_CTS!="")tc=tc+"&BSCTS=F";
		tc=tc+"&UID="+_TRK_AUID+"&SID="+_TRK_ASID;
		//2016.01.07. k3363. About DMP
		tc=tc+"&DPUID="+_TRK_DPUID;

		//instant2002 채윤식
		var bPNF=((typeof _TRK_PI)!="undefined" && _TRK_PI=="PNF")?true:false;
		if(!bPNF)tc=tc+"&js=Y";

		var ce=navigator.cookieEnabled?tc=tc+"&ce=Y":tc=tc+"&ce=N";
	    var je=navigator.javaEnabled()?tc=tc+"&je=Y":tc=tc+"&je=N";
		var _trk_bJS12=(window.screen)?true:false;
		var ss=""; var cd = "";
		   if(_trk_bJS12) {
		       ss=screen.width+"x"+screen.height;
		       cd=screen.colorDepth;
		       tc=tc+"&cd="+cd+"&ss="+ss;
		   }

		var dt=document.title.toString();
		dt=dt.substr(0,128);

		tc=tc+"&dr="+_tcts_escape(dr)+"&XDR=&du="+_tcts_escape(du)+"&dt="+_trk_escape(dt);
		tc=tc+"&tzo="+tzo+"&tye="+tye+"&tmo="+tmo+"&tda="+tda+"&tho="+tho+"&tmi="+tmi+"&tse="+tse;
		if((typeof _TRK_PI)!="undefined"&&_TRK_PI!="")tc=tc+"&PI="+_TRK_PI;
		if((typeof _TRK_CP)!="undefined" && _TRK_CP!="") tc=tc+"&CP="+_TRK_CP+"&XCP=";
		if((typeof _TRK_PN)!="undefined" && _TRK_PN!="") tc=tc+"&PN="+_trk_escape(_TRK_PN);
		if((typeof _TRK_MF)!="undefined" && _TRK_MF!="") tc=tc+"&MF="+_trk_escape(_TRK_MF);
		if((typeof _TRK_IK)!="undefined" && _TRK_IK!="") tc=tc+"&IK="+_trk_escape(_TRK_IK);
		if((typeof _TRK_CN)!="undefined" && _TRK_CN!="") tc=tc+"&CN="+_trk_escape(_TRK_CN);
		if((typeof _TRK_OA)!="undefined"&&_TRK_OA!="")tc=tc+"&OA="+_TRK_OA;
		if((typeof _TRK_OE)!="undefined"&&_TRK_OE!="")tc=tc+"&OE="+_TRK_OE;
		if((typeof _TRK_OP)!="undefined"&&_TRK_OP!="")tc=tc+"&OP="+_TRK_OP;
		if((typeof _TRK_ODN)!="undefined"&&_TRK_ODN!="")tc=tc+"&ODN="+_TRK_ODN;
		if((typeof _HMS_SX)!="undefined"&&_HMS_SX!="")tc=tc+"&HSX="+_HMS_SX;
		if((typeof _HMS_XYA)!="undefined"&&_HMS_XYA!="")tc=tc+"&HXY="+_HMS_XYA;
		if((typeof _HMS_CR)!="undefined"&&_HMS_CR!="")tc=tc+"&HCR="+_HMS_CR;
		if((typeof _TRK_AVN)!="undefined"&&_TRK_AVN>=0)tc=tc+"&VN="+_TRK_AVN;
		if((typeof _TRK_CR)!="undefined"&&_TRK_CR!="")tc=tc+"&CR="+_tcts_escape(_TRK_CR)+"&XCR=1";
		if((typeof _TRK_CQ)!="undefined"&&_TRK_CQ!="")tc=tc+"&CQ="+_tcts_escape(_TRK_CQ)+"&XCQ=1";
		if((typeof _TRK_RK)!="undefined" && _TRK_RK!="") tc=tc+"&RK="+_trk_escape(_TRK_RK);
		if((typeof _TRK_BT)!="undefined"&&_TRK_BT>=0)tc=tc+"&BT="+_TRK_BT;
		if((typeof _TRK_LIFE)!="undefined"&&_TRK_LIFE>=0)tc=tc+"&TLF="+_TRK_LIFE;
		if((typeof _TRK_CTSP)!="undefined"&&_TRK_CTSP!="")tc=tc+"&CTSP="+_tcts_escape(_TRK_CTSP);
		if((typeof _TRK_RV)!="undefined" && _TRK_RV!="") tc=tc+"&RV="+_TRK_RV;
		if((typeof _TRK_PRC)!="undefined" && _TRK_PRC!="") tc=tc+"&TOA="+_TRK_PRC;
		if((typeof _TRK_G1)!="undefined" && _TRK_G1!="") tc=tc+"&GL1="+_TRK_G1;
		if((typeof _TRK_G2)!="undefined" && _TRK_G2!="") tc=tc+"&GL2="+_TRK_G2;
		if((typeof _TRK_G3)!="undefined" && _TRK_G3!="") tc=tc+"&GL3="+_TRK_G3;
		if((typeof _TRK_G4)!="undefined" && _TRK_G4!="") tc=tc+"&GL4="+_TRK_G4;
		if((typeof _TRK_G5)!="undefined" && _TRK_G5!="") tc=tc+"&GL5="+_TRK_G5;
		if((typeof _TRK_PIV1)!="undefined"&&_TRK_PIV1!="")tc=tc+"&PIV1="+_TRK_PIV1;
		if((typeof _TRK_PIV2)!="undefined"&&_TRK_PIV2!="")tc=tc+"&PIV2="+_TRK_PIV2;
		if((typeof _TRK_PIV3)!="undefined"&&_TRK_PIV3!="")tc=tc+"&PIV3="+_TRK_PIV3;
		if((typeof _TRK_PIV4)!="undefined"&&_TRK_PIV4!="")tc=tc+"&PIV4="+_TRK_PIV4;
		if((typeof _TRK_PIV5)!="undefined"&&_TRK_PIV5!="")tc=tc+"&PIV5="+_TRK_PIV5;
		if((typeof _TRK_VISIT_NEW)!="undefined"&&_TRK_VISIT_NEW!="")tc=tc+"&visit_new="+_TRK_VISIT_NEW;
		return tc;
	}
	var _TRK_AEX="",_TRK_AUID="",_TRK_AVN="",_TRK_ASID="",_TRK_CR="",_TRK_CQ="",_HMS_XYA="";_HMS_CR="",_HMS_SX="";
	//2016.01.07. k3363. About DMP
	var _TRK_DPUID="",_TRK_A1="",_TRK_B1="";
	function getGuuidForBizspring(skeletonUrl, callback) {
		try{
			guuid = _tcts_getCookie("_BS_GUUID");
			if(guuid == "") {
				var callbackName = 'bizSpring_callback_' + Math.round(100000 * Math.random());
				window[callbackName] = function(data) {
					try{delete window[callbackName];}catch(sTrkErr){}
					document.body.removeChild(script);
					callback(data);
				};
				var script = document.createElement('script');
				script.onerror = function(){
					guuid = ((typeof _TRK_AUID)!="undefined" && _TRK_AUID!="") ? _TRK_AUID : "";
					_TRK_DPUID=guuid;
					_trk_callTracker(_TRK_A1, _TRK_B1);
				};
				script.src = skeletonUrl + (skeletonUrl.indexOf('?') >= 0 ? '&' : '?') + 'callback=' + callbackName;
				document.body.appendChild(script);
			} else {
				_TRK_DPUID=guuid;
				_trk_callTracker(_TRK_A1, _TRK_B1);
			}
		}catch(trkEr1){
			guuid = ((typeof _TRK_AUID)!="undefined" && _TRK_AUID!="") ? _TRK_AUID : "";
			_TRK_DPUID=guuid;
			_trk_callTracker(_TRK_A1, _TRK_B1);
		}
	}
	var guuidDom = "gu.bizspring.net";
	var guuidSetterURL = "";
	guuidSetterURL = (document.location.protocol.indexOf("https")!=-1?"https://":"http://")+guuidDom+'/p.php?guuid='+_tcts_getNewSID(32)+ "&domain="+document.domain ;

	function _trk_send(a1,b1){
		_TRK_A1=a1;
		_TRK_B1=b1;
		getGuuidForBizspring(guuidSetterURL, function(data) {
			guuid = data;
			_tcts_setCookie("_BS_GUUID", guuid, 30*365*24*60*60*1000);
			_TRK_DPUID=guuid;
			_trk_callTracker(_TRK_A1, _TRK_B1);
		});
	}

	function _trk_callTracker(a1,b1){
		_TRK_LAID=a1,_TRK_CTS=b1;
		if((typeof _L_LALT)!="undefined") _TRK_LIFE=_L_LALT;
		else _TRK_LIFE=30;
		_TRK_LIFE=parseInt(_TRK_LIFE)*24*60*60*1000;
		if((typeof _TRK_CTSP)=="undefined") var _TRK_CTSP="BSCCN1";
		if((typeof _TAD_CKDOM)=="undefined") _TAD_CKDOM="";
		var _SS_LIFE=30*60*1000;
		_TRK_AUID=_tcts_getCookie("_TRK_AUIDA_"+_TRK_LAID).split(":")[0];
		_TRK_AVN=_tcts_getCookie("_TRK_AUIDA_"+_TRK_LAID).split(":")[1];
		_TRK_ASID=_tcts_getCookie("_TRK_ASID_"+_TRK_LAID);

        _HMS_XYA=_tcts_getStorage("_HMS_XYA",'');
        _HMS_CR=_tcts_getStorage("_HMS_CR",'');
		if((typeof _TRK_SX)!="undefined"&&_TRK_SX!="") _HMS_SX=_TRK_SX;
		if((typeof _HMS_SX)=="undefined"||_HMS_SX=="")_HMS_SX=_tcts_getStorage("_HMS_SX",'');
		if((typeof _HMS_SX)!="undefined"&&_HMS_SX!="")_tcts_setStorage("_HMS_SX",_HMS_SX,30*365*24*60*60*1000,'','');

		if(_TRK_AUID=="") _TRK_AUID=_tcts_getNewSID(32);
		if(_TRK_AVN==""||_TRK_AVN=="NaN"||typeof(_TRK_AVN)=="undefined") _TRK_AVN=0;
		else _TRK_AVN=parseInt(_TRK_AVN);
		if(_TRK_ASID==""){
			_TRK_ASID=_tcts_getNewSID(32);
			_TRK_AVN++;
			_TRK_VISIT_NEW = "Y";
		}

		_tcts_setCookie("_TRK_AUIDA_"+_TRK_LAID, _TRK_AUID+":"+_TRK_AVN.toString(),30*365*24*60*60*1000);
		_tcts_setCookie("_TRK_ASID_"+_TRK_LAID,_TRK_ASID,_SS_LIFE);

		var dr=self.document.referrer;
		var tdu="";
		try{ tdu=top.document.location.href; }catch(_e){}
		var _TRK_REF=(dr==tdu?top.document.referrer:self.document.referrer);
		var _TRK_QST=(dr==tdu?top.document.location.search:self.document.location.search);
        _TRK_QST=_tcts_shortenURL(_TRK_QST);
        _TRK_REF=_tcts_shortenURL(_TRK_REF);
        _TRK_RV = "";
        var thisDomain = _tcts_getRootDomain(self.document.location.href);
        var _PF = true;
		if(_TRK_CTS=="Y"||_TRK_CTS=="CTS"){
            _TRK_AEX=_tcts_getCookie("_TRK_AEX_"+_TRK_LAID);
            if( (_TRK_AEX==""||_TRK_AEX=="NaN") ) {
                if(_TRK_QST.indexOf("BSPRG")>0||_TRK_QST.indexOf(_TRK_CTSP)>0) {
                    if(_TRK_AEX==""||_TRK_AEX=="NaN") _TRK_AEX=0;
                    _TRK_AEX++;
                    _tcts_setCookie("_TRK_AEX_"+_TRK_LAID,_TRK_AEX,_SS_LIFE);
                }
            } else if( parseInt(_TRK_AEX)>0 ) {
                 _TRK_AEX=parseInt(_TRK_AEX)+1;
                _tcts_setCookie("_TRK_AEX_"+_TRK_LAID,_TRK_AEX,_SS_LIFE);
            }
			if(_TRK_AEX==1 && (_TRK_QST.indexOf("BSPRG")>0||_TRK_QST.indexOf(_TRK_CTSP)>0) ) {
                if(_TRK_REF!="")_tcts_setStorage("_TRK_ACR",_TRK_REF,_TRK_LIFE,_TRK_LAID,_TAD_CKDOM);
                if(_TRK_QST!="")_tcts_setStorage("_TRK_ACQ",_TRK_QST,_TRK_LIFE,_TRK_LAID,_TAD_CKDOM);
			} else if(_TRK_AEX>1 && (_TRK_QST.indexOf("BSPRG")>0||_TRK_QST.indexOf(_TRK_CTSP)>0)){
                var _TRK_REF_LC = _TRK_REF.toLowerCase();
                if(_TRK_REF_LC.indexOf(thisDomain)<0) {
                    if(_TRK_REF!="")_tcts_setStorage("_TRK_ACR",_TRK_REF,_TRK_LIFE,_TRK_LAID,_TAD_CKDOM);
                    if(_TRK_QST!="")_tcts_setStorage("_TRK_ACQ",_TRK_QST,_TRK_LIFE,_TRK_LAID,_TAD_CKDOM);
                    _TRK_RV = "Y";
                }
            }
			_TRK_CR=_tcts_getStorage("_TRK_ACR",_TRK_LAID);
			_TRK_CQ=_tcts_getStorage("_TRK_ACQ",_TRK_LAID);
		}else if(_TRK_CTS!=""){
            _TRK_AEX=_tcts_getCookie("_TRK_AEXA_"+_TRK_LAID);
            if( (_TRK_AEX==""||_TRK_AEX=="NaN") ) {
                if( _TRK_QST.indexOf("BSCPN="+_TRK_CTS)>0 ) {
                     _TRK_AEX=1;
                    _tcts_setCookie("_TRK_AEXA_"+_TRK_LAID,_TRK_AEX,_SS_LIFE);
                }
            } else if( parseInt(_TRK_AEX)>0 ) {
                 _TRK_AEX=parseInt(_TRK_AEX)+1;
                _tcts_setCookie("_TRK_AEXA_"+_TRK_LAID,_TRK_AEX,_SS_LIFE);
            }
			if(_TRK_AEX==1 && _TRK_QST.indexOf("BSCPN="+_TRK_CTS)>0){
                if(_TRK_QST!="")_PF=_tcts_setCCA("_TRK_ACQA",_TRK_QST,_TRK_LIFE,_TRK_LAID,_TAD_CKDOM);
                if(_PF && _TRK_REF!="")_tcts_setCCA("_TRK_ACRA",_TRK_REF,_TRK_LIFE,_TRK_LAID,_TAD_CKDOM);
			} else if(_TRK_AEX>1 && _TRK_QST.indexOf("BSCPN="+_TRK_CTS)>0){
                var _TRK_REF_LC = _TRK_REF.toLowerCase();
                if(_TRK_REF_LC.indexOf(thisDomain)<0) {
                    if(_TRK_QST!="")_PF=_tcts_setCCA("_TRK_ACQA",_TRK_QST,_TRK_LIFE,_TRK_LAID,_TAD_CKDOM);
                    if(_PF && _TRK_REF!="")_tcts_setCCA("_TRK_ACRA",_TRK_REF,_TRK_LIFE,_TRK_LAID,_TAD_CKDOM);
                    _TRK_RV = "Y";
                }
			}
			_TRK_CR=_tcts_getCCA("_TRK_ACRA",_TRK_LAID,_TAD_CKDOM);
			_TRK_CQ=_tcts_getCCA("_TRK_ACQA",_TRK_LAID,_TAD_CKDOM);
		}else{
            _TRK_AEX=_tcts_getCookie("_TRK_AEXB_"+_TRK_LAID);
            if(_TRK_AEX==""||_TRK_AEX=="NaN") _TRK_AEX=0;
            _TRK_AEX++;
            _tcts_setCookie("_TRK_AEXB_"+_TRK_LAID,_TRK_AEX,_SS_LIFE);
            _tcts_setStorage("_TRK_ACQB","",_TRK_LIFE,_TRK_LAID,_TAD_CKDOM);
            _tcts_setStorage("_TRK_ACRB","",_TRK_LIFE,_TRK_LAID,_TAD_CKDOM);
		}

		//2016.01.07. k3363. About DMP For NHN
		var _tcts_code_nhn="//adlc-exchange.toast.com/sendid?sid=bizspring.co.kr&uid="+_TRK_DPUID;
		window["_tcts_img_nhn_"+_TRK_LAID+_TRK_CTS]=new Image();
		//2017.04.01. k3363. About DMP For Nas
		var _tcts_code_nas=document.location.protocol.indexOf("https")!=-1?"//sync.admixer.co.kr:4450/idsync?pid=2&uid="+_TRK_DPUID:"//sync.admixer.co.kr:8100/idsync?pid=2&uid="+_TRK_DPUID;
		window["_tcts_img_nas_"+_TRK_LAID+_TRK_CTS]=new Image();
		//2017.11.08. k3363. About DMP For SKP
		var _skp_return_url = document.location.protocol.indexOf("https")!=-1?"https":"http";
		_skp_return_url = encodeURIComponent(_skp_return_url + "://tm.bizspring.co.kr/skp.php?bzuid="+_TRK_DPUID+"&skpuid=%%SKPDMP_UID%%");
		var _tcts_code_skp="//idm.skplanet.com/pixel?nid=10&uid="+_TRK_DPUID+"&url="+_skp_return_url;
		window["_tcts_img_skp_"+_TRK_LAID+_TRK_CTS]=new Image();


		var _TRK_BT=new Date().getTime();
		window["_tcts_img_base_"+_TRK_LAID+_TRK_CTS]=new Image();
		var _tcts_code_base_SN="_tcts_code_base_"+_TRK_LAID+_TRK_CTS;
		_tcts_code_base_SN=_tcts_make_code(_TRK_LAID);

		if(_TRK_CTS=="Y"||_TRK_CTS=="CTS"){
            if(_TRK_CQ.indexOf("BSPRG")>0||_TRK_CQ.indexOf(_TRK_CTSP)>0||((typeof _TRK_PI)!="undefined"&&_TRK_PI!="")||
				((typeof _TRK_G1)!="undefined"&&_TRK_G1!="")||((typeof _TRK_G2)!="undefined"&&_TRK_G2!="")||((typeof _TRK_G3)!="undefined"&&_TRK_G3!="")||
				((typeof _TRK_G4)!="undefined"&&_TRK_G4!="")||((typeof _TRK_G5)!="undefined"&&_TRK_G5!="")||
				((typeof _TRK_PIV1)!="undefined"&&_TRK_PIV1!="")||((typeof _TRK_PIV2)!="undefined"&&_TRK_PIV2!="")||
				((typeof _TRK_PIV3)!="undefined"&&_TRK_PIV3!="")||((typeof _TRK_PIV4)!="undefined"&&_TRK_PIV4!="")||
				((typeof _TRK_PIV5)!="undefined"&&_TRK_PIV5!="")){
                if(_trk_bJS12==true){
                	if(typeof _TRK_DPUID!="undefined"&&_TRK_DPUID!=""){
						//2016.01.07. k3363. About DMP For NHN
						setTimeout(function(){window["_tcts_img_nhn_"+_TRK_LAID+_TRK_CTS].src=_tcts_code_nhn;},10);
						//2017.04.01. k3363. About DMP For Nas
						// **** setTimeout(function(){window["_tcts_img_nas_"+_TRK_LAID+_TRK_CTS].src=_tcts_code_nas;},10);
						//2017.11.08. k3363. About DMP For SKP
						setTimeout(function(){window["_tcts_img_skp_"+_TRK_LAID+_TRK_CTS].src=_tcts_code_skp;},10);
                	}
                    if(_trk_bMSIE){
                        setTimeout(function(){window["_tcts_img_base_"+_TRK_LAID+_TRK_CTS].src=_tcts_code_base_SN;},10);
                    }else{
                        setTimeout(function(){window["_tcts_img_base_"+_TRK_LAID+_TRK_CTS].src=_tcts_code_base_SN;},10);
                    }
                }else{
                    if(_trk_bMSIE)document.write('<div style=\"display: none\">');
                    document.write('<img src=\"'+_tcts_code_base_SN+'\" height=\"0\" width=\"0\">');
                    if(typeof _TRK_DPUID!="undefined"&&_TRK_DPUID!=""){
						//2016.01.07. k3363. About DMP For NHN
						document.write('<img src=\"'+_tcts_code_nhn+'\" height=\"0\" width=\"0\">');
						//2017.04.01. k3363. About DMP For Nas
						// **** document.write('<img src=\"'+_tcts_code_nas+'\" height=\"0\" width=\"0\">');
						//2017.11.08. k3363. About DMP For SKP
						document.write('<img src=\"'+_tcts_code_skp+'\" height=\"0\" width=\"0\">');
                    }

                    if(_trk_bMSIE)document.write('<\/div>');
                }
            }
		}else if(_TRK_CTS!=""){
            if(_TRK_CQ.indexOf("BSCPN="+_TRK_CTS)>0||((typeof _TRK_PI)!="undefined"&&_TRK_PI!="")||
				((typeof _TRK_G1)!="undefined"&&_TRK_G1!="")||((typeof _TRK_G2)!="undefined"&&_TRK_G2!="")||((typeof _TRK_G3)!="undefined"&&_TRK_G3!="")||
				((typeof _TRK_G4)!="undefined"&&_TRK_G4!="")||((typeof _TRK_G5)!="undefined"&&_TRK_G5!="")||
				((typeof _TRK_PIV1)!="undefined"&&_TRK_PIV1!="")||((typeof _TRK_PIV2)!="undefined"&&_TRK_PIV2!="")||
				((typeof _TRK_PIV3)!="undefined"&&_TRK_PIV3!="")||((typeof _TRK_PIV4)!="undefined"&&_TRK_PIV4!="")||
				((typeof _TRK_PIV5)!="undefined"&&_TRK_PIV5!="")){
                if(_trk_bJS12==true){
                	if(typeof _TRK_DPUID!="undefined"&&_TRK_DPUID!=""){
						//2016.01.07. k3363. About DMP For NHN
						setTimeout(function(){window["_tcts_img_nhn_"+_TRK_LAID+_TRK_CTS].src=_tcts_code_nhn;},10);
						//2017.04.01. k3363. About DMP For Nas
						// **** setTimeout(function(){window["_tcts_img_nas_"+_TRK_LAID+_TRK_CTS].src=_tcts_code_nas;},10);
						//2017.11.08. k3363. About DMP For SKP
						setTimeout(function(){window["_tcts_img_skp_"+_TRK_LAID+_TRK_CTS].src=_tcts_code_skp;},10);
                	}

                    if(_trk_bMSIE){
                        window["_tcts_img_base_"+_TRK_LAID+_TRK_CTS].src=_tcts_code_base_SN;
                    }else{
                        window["_tcts_img_base_"+_TRK_LAID+_TRK_CTS].src=_tcts_code_base_SN;
                    }
                }else{
                    if(_trk_bMSIE)document.write('<div style=\"display: none\">');
                    document.write('<img src=\"'+_tcts_code_base_SN+'\" height=\"0\" width=\"0\">');
                    if(typeof _TRK_DPUID!="undefined"&&_TRK_DPUID!=""){
						//2016.01.07. k3363. About DMP For NHN
						document.write('<img src=\"'+_tcts_code_nhn+'\" height=\"0\" width=\"0\">');
						//2017.04.01. k3363. About DMP For Nas
						// **** document.write('<img src=\"'+_tcts_code_nas+'\" height=\"0\" width=\"0\">');
						//2017.11.08. k3363. About DMP For SKP
						document.write('<img src=\"'+_tcts_code_skp+'\" height=\"0\" width=\"0\">');
                    }

                    if(_trk_bMSIE)document.write('<\/div>');
                }
            }
		}else{
            if(_TRK_CR!=""||_TRK_CQ!=""||((typeof _TRK_PI)!="undefined"&&_TRK_PI!="")||
				((typeof _TRK_G1)!="undefined"&&_TRK_G1!="")||((typeof _TRK_G2)!="undefined"&&_TRK_G2!="")||((typeof _TRK_G3)!="undefined"&&_TRK_G3!="")||
				((typeof _TRK_G4)!="undefined"&&_TRK_G4!="")||((typeof _TRK_G5)!="undefined"&&_TRK_G5!="")||
				((typeof _TRK_PIV1)!="undefined"&&_TRK_PIV1!="")||((typeof _TRK_PIV2)!="undefined"&&_TRK_PIV2!="")||
				((typeof _TRK_PIV3)!="undefined"&&_TRK_PIV3!="")||((typeof _TRK_PIV4)!="undefined"&&_TRK_PIV4!="")||
				((typeof _TRK_PIV5)!="undefined"&&_TRK_PIV5!="")){
                if(_trk_bJS12==true){
                	if(typeof _TRK_DPUID!="undefined"&&_TRK_DPUID!=""){
						//2016.01.07. k3363. About DMP For NHN
						setTimeout(function(){window["_tcts_img_nhn_"+_TRK_LAID+_TRK_CTS].src=_tcts_code_nhn;},10);
						//2017.04.01. k3363. About DMP For Nas
						// **** setTimeout(function(){window["_tcts_img_nas_"+_TRK_LAID+_TRK_CTS].src=_tcts_code_nas;},10);
						//2017.11.08. k3363. About DMP For SKP
						setTimeout(function(){window["_tcts_img_skp_"+_TRK_LAID+_TRK_CTS].src=_tcts_code_skp;},10);
                	}

                    if(_trk_bMSIE){
                        setTimeout(function(){window["_tcts_img_base_"+_TRK_LAID+_TRK_CTS].src=_tcts_code_base_SN;},10);
                    }else{
                        setTimeout(function(){window["_tcts_img_base_"+_TRK_LAID+_TRK_CTS].src=_tcts_code_base_SN;},10);
                    }
                }else{
                    if(_trk_bMSIE)document.write('<div style=\"display: none\">');
                    document.write('<img src=\"'+_tcts_code_base_SN+'\" height=\"0\" width=\"0\">');
                    if(typeof _TRK_DPUID!="undefined"&&_TRK_DPUID!=""){
						//2016.01.07. k3363. About DMP For NHN
						document.write('<img src=\"'+_tcts_code_nhn+'\" height=\"0\" width=\"0\">');
						//2017.04.01. k3363. About DMP For Nas
						// **** document.write('<img src=\"'+_tcts_code_nas+'\" height=\"0\" width=\"0\">');
						//2017.11.08. k3363. About DMP For SKP
						document.write('<img src=\"'+_tcts_code_skp+'\" height=\"0\" width=\"0\">');
                    }

                    if(_trk_bMSIE)document.write('<\/div>');
                }
            }
        }
		// tame : begin
		window["_tcts_img_tame"]=new Image();
		var _tcts_code_base_tame = _tcts_code_base_SN.replace("ssl.logger.co.kr/tracker_L4Ad.tsp", "tm.bizspring.co.kr/trk_cts.php") + "&fromTag=Y&GA="+encodeURIComponent(_tcts_getCookie("_ga"));
		if(_trk_bJS12==true){
			setTimeout(function(){window["_tcts_img_tame"].src=_tcts_code_base_tame;},10);
		} else {
			if(_trk_bMSIE)document.write('<div style=\"display: none\">');
			document.write('<img src=\"'+_tcts_code_base_tame+'\" height=\"0\" width=\"0\">');
			if(_trk_bMSIE)document.write('<\/div>');
		}
		// tame : end
	}
	var G="apply",C="call",z="prototype",Qc="replace",t="indexOf";
	var aa=encodeURIComponent,f=window,ba=setTimeout,n=Math,ea=RegExp;
	var O=f,M=document,ua=function(a,b,c){a.removeEventListener?a.removeEventListener(b,c,!1):a.detachEvent&&a.detachEvent("on"+b,c)};
	var ta=function(a,b,c,d){try{a.addEventListener?a.addEventListener(b,c,!!d):a.attachEvent&&a.attachEvent("on"+b,c)}catch(e){}};
	function qa(a){return void 0!=a&&-1<(a.constructor+"")[t]("String")}
	var ld;
	if(ld=qa(f.BSAnalyticsObj)){
		var md=f.BSAnalyticsObj;
		ld=md?md[Qc](/^[\s\xa0]+|[\s\xa0]+$/g,""):"";
	}
	var gb=ld||"_tcts_m";
	var rc=function(a){
		if("prerender"==M.webkitVisibilityState)return!1;
		a();
		return!0;
	};
	var Mc=function(a){
		if(!rc(a)){
			var b=!1,c=function(){!b&&rc(a)&&(b=!0,ua(M,"webkitvisibilitychange",c))};
			ta(M,"webkitvisibilitychange",c)
		}
	};
	var BA={F:"/plugins/ua/"};
	BA.D=function(a){
		for(var i=0;i<arguments.length;i++){
			_trk_send(arguments[i][0],arguments[i][1]);
		}
	};
	var $=function(){_trk_send(arguments[0],arguments[1])};
	$.B=function(){
		var a=O[gb];O[gb]=$;var b=a&&a.q;"[object Array]"==Object[z].toString[C](Object(b))&&Mc(function(){BA.D[G]($,b)})
	};
	$.B();
})(window);
