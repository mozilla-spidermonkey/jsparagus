/**
 * Qboot.load v0.7
 * lite版 去除引入核心文件自动加载机制
 * Date:2013.3.19
 * Copyright 2012, irideas & 月影
 *
 * CHANGE LOG:
 * 03.19 2013 第一个参数是方法失败的情况
 * 06.11 2012 document.getElementsByTagName('script')[0].hasOwnProperty("parentNode"); Chrome18返回TRUE,坑爹啊
 * 12.09 2011 加入done方法 外部可设置该模块状态为完成
 * 11.16 2011 qboot.add 配置中加入force属性 强行拉取
 * 05.09 2011 qboot.load.css 支持传入第二个可选参数 默认为:inline-css-id
 *
 * BUG :
 * load.css 在HEAD中使用 IE6会中止加载 某些特殊条件下...(都遗忘了)
 *

	* TODO:
	* 场景load(url) 需要用一个通用配置 如charset等
	* load.css(string,id) 对同一ID的使用有时不需要追加 而是剔除
	* 【BUG】load({path:"http://tuan.360.cn/scripts/jquery-1.4.4.min.js",type:"js"},{path:"http://tuan.360.cn/scripts/jquery-1.4.4.min.js",type:"js"},{path:"http://tuan.360.cn/scripts/jquery-1.4.4.min.js",type:"js"})
		多个对象作为参数 第二个起失效
	
	* USAGE:
	* var qload = qboot.load;

	* [1] qload("http://tuan.360.cn/scripts/jquery-1.4.4.min.js"); 
	* //直接调用外部JS

	* [2] qload("http://tuan.360.cn/styles/reset.css");
	* //直接调用外部CSS
	
	* [3] qload.add('lightBoxCss', {path: 'http://leandrovieira.com/projects/jquery/lightbox/css/jquery.lightbox-0.4.css', type: 'css'});
	* //定义lightBoxCss 配置 path:'文件地址',type:'css'

	* [4] qload.add('lightBoxJs', {path: 'http://leandrovieira.com/projects/jquery/lightbox/js/jquery.lightbox-0.4.js', type: 'js', requires: ['lightBox-css']});
	* //定义lightBoxJs  配置 path:'文件地址',type:'js',requires['定义1','定义2'],force:true/false（是否每次强行拉取 默认:false） 

	* [5] qload('lightBoxJs', function(){});
	* //引用lightBoxJs 

	* [6] qload.done('lightBoxJs');
	* //当配置lightBoxJs不强行拉取时，可在某些时机设置以完成

	* [7] qload.css(".dialog .hd h3 { margin:0; }.dialog-close:link { text-decoration:none; }","qboot-inline-css");
	* //直接插入CSS代码,参数2为指定ID
	
	* [8] qload.delay(2000,'lightBoxJs',function() {});
	* //延期执行或延期加载
	
	* [9] qload({name: 'jQuery1.4.4', path:"http://tuan.360.cn/scripts/jquery-1.4.4.min.js",type:"js"}, callback);
	* //直接用配置的方式加载js
	*
	* [10] qload(function(){});
	* //塞入方法
**/var qload=qload||{};qload=function(){var e=document,t=window,n={},r={},i=function(e){return e.constructor===Array},s={mods:{}},o=e.getElementsByTagName("script"),u=o[o.length-1],a,f=function(e){if(e.clearAttributes)e.clearAttributes();else for(var t in e)e.hasOwnProperty(t)&&t.toLowerCase()!=="parentnode"&&delete e[t];e&&e.parentNode&&e.parentNode.removeChild(e),e=null},l=e.createElement("script").readyState?function(e,t){e.onreadystatechange=function(){var n=e.readyState;if(n==="loaded"||n==="complete")e.onreadystatechange=null,t.apply(this)}}:function(e,t){e.addEventListener("load",t,!1)},c=function(t,i,s,o,a,h){var p=u;if(!t)return;if(n[t]){r[t]=!1;if(!o){a&&a(t,h);return}}if(r[t]){setTimeout(function(){c(t,i,s,o,a,h)},1);return}r[t]=!0;var d;if(i==="js"||t.indexOf(".js")>=0)d=e.createElement("script"),d.setAttribute("type","text/javascript"),s&&(d.charset=s),d.setAttribute("src",t),d.setAttribute("async",!0),l(d,function(){n[t]=!0,a&&a(t,h),f(d)}),p.parentNode.insertBefore(d,p);else if(i==="css"||t.indexOf(".css")>=0){d=e.createElement("link"),d.setAttribute("type","text/css"),s&&(d.charset=s),d.setAttribute("rel","stylesheet"),d.setAttribute("href",t),n[t]=!0,p.parentNode.insertBefore(d,p),a&&a(t,h);return}},h=function(e){if(!e||!i(e))return;var t=0,n,r=[],o=s.mods,u=[],a={},f=function(e){var t=0,n,r;if(a[e])return u;a[e]=!0;if(o[e].requires){r=o[e].requires;for(;typeof (n=r[t++])!="undefined";)o[n]?(f(n),u.push(n)):u.push(n);return u}return u};for(;typeof (n=e[t++])!="undefined";)o[n]&&o[n].requires&&o[n].requires[0]&&(u=[],a={},r=r.concat(f(n))),r.push(n);return r},p=function(e){if(!e||!i(e))return;this.queue=e,this.current=null};return p.prototype={_interval:10,start:function(){var e=this;this.current=this.next();if(!this.current){this.end=!0;return}this.run()},run:function(){var e=this,t,n=this.current;if(typeof n=="function"){n(),this.start();return}typeof n=="string"&&(s.mods[n]?(t=s.mods[n],c(t.path,t.type,t.charset,t.force,function(t){e.start()},e)):/\.js|\.css/i.test(n)?c(n,"","","",function(e,t){t.start()},e):this.start())},next:function(){return this.queue.shift()}},a=function(){var e=[].slice.call(arguments),t,n=e[0];if(typeof n!="string"&&typeof n!="function"){var r=n.name||n.path,i=n.callback||e[1];a.add(r,n),e[0]=r,e[1]=i}t=new p(h(e)),t.start()},a.add=function(e,t){if(!e||!t||!t.path)return;s.mods[e]=t},a.delay=function(){var e=[].slice.call(arguments),n=e.shift();t.setTimeout(function(){a.apply(this,e)},n)},a.done=function(){var e=[].slice.call(arguments),t=0,r;for(;r=e[t++];)typeof r=="string"&&(s.mods[r]?(mod=s.mods[r],n[mod.path]=!0):/\.js|\.css/i.test(r)&&(n[r]=!0))},a.css=function(t,n){n=n||"qboot-inline-css";var r=e.getElementById(n);r||(r=e.createElement("style"),r.type="text/css",r.id=n,e.getElementsByTagName("head")[0].appendChild(r)),r.styleSheet?r.styleSheet.cssText=r.styleSheet.cssText+t:r.appendChild(e.createTextNode(t))},a}();