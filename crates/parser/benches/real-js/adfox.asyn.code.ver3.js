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
    if(AF_doc){
	   AF_doc.write('<scr'+'ipt onerror="document.close();" type="text/javascript" src="'+AF_src+'"><\/scr'+'ipt>');
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