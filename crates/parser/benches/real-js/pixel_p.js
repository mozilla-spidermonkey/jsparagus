function makeGetParameterAdd(name,val) {
	return ((val != null && val.toString().length > 0) ? '&'+name+'='+encodeURIComponent(val) : '');
}
document.write('<sc'+'ript type="text/javascript" src="//px.ladsp.com/pixel_p?advertiser_id='+smnAdvertiserId
+ ((typeof(smnRetargetingParameter) !== "undefined") ? makeGetParameterAdd('rp',smnRetargetingParameter) : '')
+ ((typeof(smnLogParameter1)  !== "undefined") ? makeGetParameterAdd('lp1',smnLogParameter1) : '')
+ ((typeof(smnLogParameter2)  !== "undefined") ? makeGetParameterAdd('lp2',smnLogParameter2) : '')
+ ((typeof(smnLogParameter3)  !== "undefined") ? makeGetParameterAdd('lp3',smnLogParameter3) : '')
+ ((typeof(smnLogParameter4)  !== "undefined") ? makeGetParameterAdd('lp4',smnLogParameter4) : '')
+ ((typeof(smnLogParameter5)  !== "undefined") ? makeGetParameterAdd('lp5',smnLogParameter5) : '')
+ ((typeof(smnLogParameter6)  !== "undefined") ? makeGetParameterAdd('lp6',smnLogParameter6) : '')
+ ((typeof(smnLogParameter7)  !== "undefined") ? makeGetParameterAdd('lp7',smnLogParameter7) : '')
+ ((typeof(smnLogParameter8)  !== "undefined") ? makeGetParameterAdd('lp8',smnLogParameter8) : '')
+ ((typeof(smnLogParameter9)  !== "undefined") ? makeGetParameterAdd('lp9',smnLogParameter9) : '')
+ ((typeof(smnLogParameter10) !== "undefined") ? makeGetParameterAdd('lp10',smnLogParameter10) : '')
+ '&referer='+encodeURIComponent(document.referrer.toString())
+ '"></sc'+'ript>');
