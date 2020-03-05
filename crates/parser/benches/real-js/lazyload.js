var JsLazyLoad ={
		loop : 0,
		scriptList : [],
		load: function(srcList) {
		var self =this;
	    for(var i = 0 ; i < srcList.length;i++){
	    	var script = document.createElement('script');
	    	script.setAttribute('type', 'text/javascript');
	   		script.src = srcList[i];
	   		self.scriptList[i] = script;
	    }
	    
	    for(var i = 0 ; i < self.scriptList.length-1;i++){
    		if(self.scriptList[i].readyState) {
    			self.scriptList[i].onreadystatechange = function(){ 	
    				if(!self.scriptList[self.loop+1]) return;
        	    	if(self.scriptList[self.loop].readyState==="loaded" || self.scriptList[self.loop].readyState==="complete"){
	    				document.getElementsByTagName('body')[0].appendChild(self.scriptList[self.loop+1]);
        				self.loop++;
        	    	}
    			};
    		}else{
	    		self.scriptList[i].addEventListener("load",function(){
	    				document.getElementsByTagName('body')[0].appendChild(self.scriptList[self.loop+1]);
	    				self.loop++;
	    			},false);
    		}
	    }
	    
	    if(self.scriptList.length >0) document.getElementsByTagName('body')[0].appendChild(self.scriptList[0]);
	}
};

var ImgLazyLoad = {
	load:function(){
		var imgList= document.getElementsByTagName('img');
		for(var i = 0 ; i< imgList.length;i++){
			var img = imgList[i].getAttribute('lazyimg');
			if(img) imgList[i].src = img ;
		}
	}
}