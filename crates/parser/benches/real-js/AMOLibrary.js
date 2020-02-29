//This file will be referenced in the main html file(and only once in the html code base), that <script> referenced will be replaced with server variables before being injected into friendly iframe.

//this is AMO library code

     if (navigator.userAgent.indexOf("MSIE") != -1 || navigator.userAgent.indexOf("Trident") != -1) {
         if (!Object.keys) {
             Object.keys = function(obj) {
                 var keys = [];
                 for (var i in obj) {
                     if (obj.hasOwnProperty(i)) {
                         keys.push(i);
                     }
                 }
                 return keys;
             };
         }
     }

    var amo;
    if (window.top === window) {
        amo = new function() {
                 var localClk = {};
                 this.variation = {};
                 this.attributes = {};
                 this.content = [];
                 var validHeaders = {"name":"1", "description":"1", "provider":"1", "brand":"1", "display_advertiser_category_name":"1", "price":"1", "discount_price":"1", "picture_url":"1", "product_url":"1", "passthroughfield1":"1", "passthroughfield2":"1", "passthroughfield3":"1", "passthroughfield4":"1", "passthroughfield5":"1", "image_url1":"1", "image_url2":"1", "image_url3":"1", "image_url4":"1", "image_url5":"1"};
                 var gid = 0;

                 function validateContent(headers) {
                     for (var i = 0; i < headers.length; i++) {
                         var header = headers[i];
                         var valid = 0;
                         if(validHeaders[header] == 1) {
                             valid=1;
                         } else {
                             alert("Invalid Header in Content: " + header);
                             break;
                         }
                     }
                     return valid;
                 };

                 this.registerClick = function(clkVar, clkUrl) {
                     localClk[clkVar] = clkUrl;
                 }

                 this.registerVariation = function(key, value) {
                     if(!key || !value) {
                         alert("ERROR: key or/and value is not passed");
                         return;
                     }
                     if((typeof key !== 'string') || (typeof value !== 'string')) {
                         alert("ERROR: key and value needs to be string");
                         return;
                     }
                     this.variation[key] = value;
                 }

                 this.registerAttribute = function(name, type, value) {
                       if(!name || !value || !type) {
                           alert("ERROR: name or/and type or/and value is not passed");
                           return;
                       }
                       if(type != 'TEXT' && type != 'IMAGE') {
                         alert("ERROR: type needs to be TEXT/IMAGE");
                         return;
                       }
                       if((typeof name !== 'string') || (typeof value !== 'string')) {
                           alert("ERROR: name and value needs to be string");
                           return;
                       }
                       this.attributes[name] = value;
                 }

                 this.registerContent = function(contentData) {
                     if(!contentData) {
                         alert("ERROR: Content Data is not passed");
                         return;
                     }
                     validHeader = true;
                     if (contentData.length == undefined) {
                         if (validateContent(Object.keys(contentData)) != 0) {
                             contentData.gid = ++gid;
                             this.content.push(contentData);
                         }
                     } else {
                         alert("Content Data needs to be a JSON Object. Passed in Content Data seems to be an Array");
                     }
                 }

                 this.onDynAdClick = function(content, event, overrideUrl, customText, clickTime) {
                     var clkUrl;
                     clkUrl = (overrideUrl ? overrideUrl : ((content) ? content.product_url : ""));
                     if ((clkUrl.length > 0) && (clkUrl.indexOf("http://") == 0 || clkUrl.indexOf("https://") == 0 || clkUrl.indexOf("//") == 0)) {
                         window.open(clkUrl);
                     } else {
                         alert("ERROR: Click URL is not available or is not valid.");
                     }
                 }

                 this.onAdClick = function(clk, event) {
                     var clkUrl;
                     if (clk) {
                         if (clk.indexOf("http://") == 0 || clk.indexOf("https://") == 0 || clk.indexOf("//") == 0) {
                             clkUrl = clk;
                         } else {
                             if (localClk[clk]) clkUrl = localClk[clk];
                         }
                         if (clkUrl) {
                             window.open(clkUrl);
                         } else {
                             alert("ERROR: Click Parameter " + clk + " not registered. Register landing page using amo.registerClick");
                         }
                     } else {
                         alert("ERROR: null or undefined Click Parameter passed");
                     }
                 }
             };

    } else {
         amo = window.parent['amo'];
    }
