function AdFox()
{
	var instance;
		if (!adfox) instance = this;
		else return adfox;
	/* 
	*@SETTINGS
	*/
		this.phoneWidth = 480;				// Макс. ширина экрана телефона
		this.tabletWidth = 830;				// Макс. ширина экрана планшета
		this.minWidth = 200;				// Минимальная ширина баннера при resize & scale 
		this.autoReloads = 1;			// Автоматическая перегрузка баннера
	/*
	*@END SETTINGS
	*/
	this.pc = 'display';	  			// Имя для мониторв
	this.pad = 'tablet';	 			// Имя для планшетов
	this.phone = 'phone';    			// Имя для телефонов
	this.rnd = function()
	{
		return Math.floor(Math.random() * 1000000);
	}
	this.pr = window.pr || this.rnd();
    window.pr = window.pr || this.pr;
	this.ref = escape(document.referrer);
	this.dl = escape(document.location);
	this.banners = [];
	this.state = '';
	this.prev = '';
	this.screen = function()
	{
		var w = this.getWidth();
			if(w > this.tabletWidth)
			{
				this.prev = this.state;
				this.state = this.pc;
			}
			else if((w < this.tabletWidth || w == this.tabletWidth) && w > this.phoneWidth)
			{
				this.prev = this.state;
				this.state = this.pad;
			}
			else if(w < this.phoneWidth || w == this.phoneWidth)
			{
				this.prev = this.state;
				this.state = this.phone;
			}
	}
	this.getWidth = function()
	{
		var win = window,
			d = document,
			e = d.documentElement,
			g = d.getElementsByTagName('body')[0],
			w = e.clientWidth || win.innerWidth || g.clientWidth;
		return w;
	}
	this.getBanner = function(pr1)
	{
		for (var i = 0; i < adfox.banners.length; i++) {
			if (adfox.banners[i].pr1 == pr1) {
				return adfox.banners[i];
			}
		}
		return adfox.banners;
	}
	this.removeBanner = function(e)
	{
		for(var i = 0; i < adfox.banners.length;i++)
		{
			if(adfox.banners[i] == e)
			{
				adfox.banners[i].ph.innerHTML = '';
				adfox.banners.splice(i,1);
			}
		}
	}
	/* ENTER FRAME*/
	this.enterframe = function(){
		var pc = this.pc,
			pad = this.pad,
			phone = this.phone,
			ds = this.displaysSize;
		function update(iter, _rnd)
		{
			iter++;
			var winSize = adfox.getWidth();
				adfox.screen();

			function removeHTMLCodeBanner(id) {
				var iframeWindow = null;
				try{
					if(document.all && !window.opera){
						iframeWindow = window.frames['AdFox_iframe_' + e.pr1];
					}
					else if(document.getElementById){
						iframeWindow = document.getElementById('AdFox_iframe_' + e.pr1).contentWindow;
					}
				}catch(e){}

				if (iframeWindow && typeof(iframeWindow.adfoxRemoveBanner) != 'undefined') {
					iframeWindow.adfoxRemoveBanner();
				}
			}

			for(var i = 0; i < adfox.banners.length; i++)
			{
				var b = adfox.banners[i];
				if(b.settings != 'none')
				{
					/* VISIBILITY */
					if(adfox.parse(pc,b.settings) || adfox.parse(pad,b.settings) || adfox.parse(phone,b.settings))
					{
						if(adfox.parse(adfox.state,b.settings) && b.ph.style.display != 'none')
						{
							b.ph.style.display = 'none';
						}
						else if(!adfox.parse(adfox.state,b.settings) && b.ph.style.display != 'block')
						{
							b.ph.style.display = 'block';
						}
					}
					
					/* RESIZE BANNER */
					if(adfox.parse('scale',b.settings)  && b.w != '')
					{
						var wid = (b.ph.parentNode.clientWidth/b.w < 1)?b.ph.parentNode.clientWidth/b.w:1;
						b.ph.style.width = Math.floor(wid*b.w)+'px';
						if(parseInt(b.ph.style.width)< adfox.minWidth)
							b.ph.style.width = Math.floor(adfox.minWidth)+'px';
					}
					
				}
				/* RELOAD */
				if(adfox.autoReloads == true && (b.ph.style.display == 'block' || b.ph.style.display == ''))
				{
					if(adfox.state != adfox.prev)
					{
						removeHTMLCodeBanner(b.pr1);
						adfox.removeBanner(b); 
						b.createBanner();
					}
				}
			}
			setTimeout(function() {update(iter, _rnd);},200);
		}
		update(0, Math.random());
	}
	this.parse = function(srch,str)
	{
		var res = false;
		var f = srch.toLowerCase();
		var s = str.toLowerCase();
		if(s.indexOf(f) != -1)
		{
			res = true;
		}
		return res;
	}
}
var adfox = new AdFox();
	adfox.enterframe();

function AdFoxBanner(link,settings)
{
	this.pr1 = adfox.rnd();
	this.ph	= null;
	this.date;
	this.iframe;
	this.settings = (settings)?settings:'none';
	this.link;
	this.w = '';
	this.getCode = function(n)
	{	
		var link = this.link;
		var pr1 = this.pr1;
		var set = this.settings;
		var n = n;
		var getWid = function()
		{
			if(adfox.banners[n].ph != null)
			{
				var img = adfox.banners[n].ph.getElementsByTagName('img');
				if(img.length == 1)
					adfox.banners[n].w = parseInt(img[0].getAttribute('width'));
				else
					adfox.banners[n].w = adfox.banners[n].ph.clientWidth;
			}
			else
			{
				setTimeout(getWid,1000);
			}			
		}		
		var timer = function()
		{
			if(set != '')
			{
				if(adfox.parse(adfox.state,set))
				{
					//No request.  
					setTimeout(timer,30);
				}
				else
				{
					AdFox_getCodeScript(1,pr1,link);
					setTimeout(getWid,1000);
				}
			}
			else
			{
				AdFox_getCodeScript(1,pr1, link);
				setTimeout(getWid,1000);
			}
		}
		timer();
	}

	this.createBanner = function()
	{
		var f = true;
		var curIndex;
		this.date = new Date();
		this.link = link;
		this.secParams = '&pr=' + adfox.pr +'&pt=b&pd=' + this.date.getDate() + '&pw=' + this.date.getDay() + '&pv=' + this.date.getHours() + '&prr=' + adfox.ref + '&dl='+adfox.dl+'&pr1='+this.pr1+'&phid=AdFox_banner_'+this.pr1;
		for(var i = 0; i < adfox.banners.length;i++)
		{
			if(adfox.banners[i] == this){ 
				f=false;
				break;
			}
		}
		if(f)
		{
			adfox.banners.unshift(this);
			curIndex = adfox.banners.length-1;
		};
		if(this.ph == '' || this.ph == null && document.readyState != 'complete')
		{
			document.write('<div id="AdFox_banner_'+this.pr1+'"><\/div>');
			document.write('<div style="visibility:hidden; position:absolute;"><iframe id="AdFox_iframe_'+this.pr1+'" width=1 height=1 marginwidth=0 marginheight=0 scrolling=no frameborder=0><\/iframe><\/div>');
		}
		this.ph = document.getElementById('AdFox_banner_'+this.pr1);
		this.iframe = document.getElementById('AdFox_iframe_'+this.pr1);
		if(this.ph != null){
			this.link = this.link.replace(/&amp;/g,'&');
			this.link += this.secParams;
			this.getCode(curIndex);
		}
	}
}
//old async v3
function AdFox_SetLayerVis(spritename,state){
   document.getElementById(spritename).style.visibility=state;
}

function AdFox_Open(AF_id){
   AdFox_SetLayerVis('AdFox_DivBaseFlash_'+AF_id, "hidden");
   AdFox_SetLayerVis('AdFox_DivOverFlash_'+AF_id, "visible");
}

function AdFox_Close(AF_id){
   AdFox_SetLayerVis('AdFox_DivOverFlash_'+AF_id, "hidden");
   AdFox_SetLayerVis('AdFox_DivBaseFlash_'+AF_id, "visible");
}

function AdFox_getCodeScript(AF_n,AF_id,AF_src){
    var AF_doc;
    if(AF_n<10){
	
      try{
	     if(document.all && !window.opera){
		    AF_doc = window.frames['AdFox_iframe_'+AF_id].document;
			}else if(document.getElementById){
			         AF_doc = document.getElementById('AdFox_iframe_'+AF_id).contentDocument;
					}
		 }catch(e){}
    if(AF_doc && AF_doc.readyState=='complete'){
		var s = document.createElement('script');
			AF_doc.write('<html><head></head><body marginwidth="0" marginheight="0"></body></html>');
			if (typeof(window.chrome) != 'undefined' || navigator.userAgent.indexOf('iPhone') >= 0 || navigator.userAgent.indexOf('iPad') >= 0 || navigator.userAgent.indexOf('iPod') >= 0) {
		   		AF_doc.location.hash = 'AdFox_fix';
		    }
		    s.onerror = function() {
				AF_doc.close();
			};
			s.src = AF_src;
			if(AF_doc.getElementsByTagName('head')[0])
				AF_doc.getElementsByTagName('head')[0].appendChild(s);
			else 
				AF_doc.appendChild(s);
	   }else{
	      setTimeout('AdFox_getCodeScript('+(++AF_n)+','+AF_id+',"'+AF_src+'");', 100);
		  	}
	}
}
function adfoxSdvigContent(banID, flashWidth, flashHeight){
	var obj = document.getElementById('adfoxBanner'+banID).style;
	if (flashWidth == '100%') obj.width = flashWidth;
	    else obj.width = flashWidth + "px";
	if (flashHeight == '100%') obj.height = flashHeight;
	    else obj.height = flashHeight + "px";
}
function adfoxVisibilityFlash(banName, flashWidth, flashHeight){
    	var obj = document.getElementById(banName).style;
	if (flashWidth == '100%') obj.width = flashWidth;
	    else obj.width = flashWidth + "px";
	if (flashHeight == '100%') obj.height = flashHeight;
	    else obj.height = flashHeight + "px";
}
function adfoxStart(banID, FirShowFlNum, constVisFlashFir, sdvigContent, flash1Width, flash1Height, flash2Width, flash2Height){
	if (FirShowFlNum == 1) adfoxVisibilityFlash('adfoxFlash1'+banID, flash1Width, flash1Height);
	    else if (FirShowFlNum == 2) {
		    adfoxVisibilityFlash('adfoxFlash2'+banID, flash2Width, flash2Height);
	        if (constVisFlashFir == 'yes') adfoxVisibilityFlash('adfoxFlash1'+banID, flash1Width, flash1Height);
		    if (sdvigContent == 'yes') adfoxSdvigContent(banID, flash2Width, flash2Height);
		        else adfoxSdvigContent(banID, flash1Width, flash1Height);
	}
}
function adfoxOpen(banID, constVisFlashFir, sdvigContent, flash2Width, flash2Height){
	var aEventOpenClose = new Image();
	var obj = document.getElementById("aEventOpen"+banID);
	if (obj) aEventOpenClose.src =  obj.title+'&rand='+Math.random()*1000000+'&prb='+Math.random()*1000000;
	adfoxVisibilityFlash('adfoxFlash2'+banID, flash2Width, flash2Height);
	if (constVisFlashFir != 'yes') adfoxVisibilityFlash('adfoxFlash1'+banID, 1, 1);
	if (sdvigContent == 'yes') adfoxSdvigContent(banID, flash2Width, flash2Height);
}
function adfoxClose(banID, constVisFlashFir, sdvigContent, flash1Width, flash1Height){
	var aEventOpenClose = new Image();
	var obj = document.getElementById("aEventClose"+banID);
	if (obj) aEventOpenClose.src =  obj.title+'&rand='+Math.random()*1000000+'&prb='+Math.random()*1000000;
	adfoxVisibilityFlash('adfoxFlash2'+banID, 1, 1);
	if (constVisFlashFir != 'yes') adfoxVisibilityFlash('adfoxFlash1'+banID, flash1Width, flash1Height);
	if (sdvigContent == 'yes') adfoxSdvigContent(banID, flash1Width, flash1Height);
}