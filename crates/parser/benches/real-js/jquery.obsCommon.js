// **********************************************************
// * Common jQuery utilities        *
// *                                                        *
// * NOTE: Only keep SMALL things in here, full plugins    *
// * should be in their own file!                           *
// **********************************************************

var obs = function ($) {
    var config = {};

    var init = function (configs) {
        config = {
            
        };
        config = obs.utils.merge(config, configs);

    };

    return { init: init };
} (jQuery);


obs.utils = function($) {

    /**
    * serializeJSON - turns JSON to string.  Uses built in
    *   JSON.stringify if available
    */
    var serializeJSON = function (obj) {
        var t = typeof (obj);
        if (t != "object" || obj === null) {
            if (t == "string") { obj = '"' + obj + '"'; }
            return obj;
        } else {
            var n, v, json = [], arr = (obj && obj.constructor == Array);
            for (n in obj) {
                if (obj.hasOwnProperty(n)) {
                    v = obj[n]; t = typeof (v);
                    if (t == "string") { v = '"' + v + '"'; }
                    else if (t == "object" && v !== null) { v = serializeJSON(v); }
                    json.push((arr ? "" : '"' + n + '":') + String(v));
                }
            }
            return (arr ? "[" : "{") + String(json) + (arr ? "]" : "}");
        }
    };

    /**
    * merge - merges two objects
    * @param {Object} original Original object
    * @param {Object} update Object to merge in.  These values overwrite.
    * @returns {Object} Merged object
    */
    var merge = function(original, update) {
        var value = '';
        for (var key in update) {
            if (update.hasOwnProperty(key)) {
                value = ((!!update[key]) && update[key].constructor == String) ? update[key].replace( /<[^>]*>/g , '') : update[key];
                original[key] = value;
            }
        }
        return original;
    };
    
    var getUrlVar = function () {
        var vars;

        /**
        * getVar - Get values from url parameters.  If no arguments are passed,
        *   object with all URL parameters is returned.
        * @param {String} key URL parameter to retrieve.
        */
        var getVar = function (key) {
            if (!key) { return getVars(); }
            return getVars()[key];
        };

        /**
        * getVars - Get object with all URL parameters
        * @private
        * @returns {Object} all URL parameters
        */
        var getVars = function () {
            if (vars) { return vars; }
            vars = {};
            var hashes;
            if (!!(hashes = window.location.href.replace(/(\?[^\?]*)\?/g, '$1&').split('?')[1])) {
                var hash;
                hashes = hashes.split('&');
                for (var i = hashes.length; i--; ) {
                    hash = hashes[i].split('=');
                    vars[hash[0]] = hash[1];
                }
            }
            return vars;
        };

        return getVar;
    } ();

    var contains = function (arr, obj) {
        if (arr.indexOf) {
            return arr.indexOf(obj) < 0;
        }
        return $.inArray(arr, obj);
    };
	
	var loadSelectOptions = function ( target, selectValues ) {
        console.info(selectValues);
		if ($(target).length) {
			$(target).find('option').remove();
			$.each(selectValues, function(key, value) {   
				key = value.value;
                if (value.text)
				    text = value.text.replace(/&amp;/g, '&');
                else 
                    text = 'Error';
				$(target)
				  .append($('<option>', { value : key })
				  .text(text)); 
			});
		}
	};

    var defaultOptionValue = function() {
        var default_object = {};
        default_object['value'] = '-1';
        default_object['text'] = '(No Selection)';
        default_objects_patch = [];
        default_objects_patch.push(default_object);

        return default_objects_patch;
    };

    var loadScript = function (url, target, callback) {

        var script = document.createElement("script")
        script.type = "text/javascript";

        if (script.readyState){  // IE
            script.onreadystatechange = function(){
                if (script.readyState == "loaded" || script.readyState == "complete") {
                    script.onreadystatechange = null;
                    if (callback) {
                        callback();
                    }
                }
            };
        } else {  // Others
            script.onload = function(){
                if (callback) {
                    callback();
                }
            };
        }

        script.src = url;
        target.appendChild(script);
    };

    return {
        serializeJSON: serializeJSON,
        merge: merge,
        getUrlVar: getUrlVar,
        contains: contains,
		loadSelectOptions: loadSelectOptions,
        defaultOptionValue: defaultOptionValue,
        loadScript: loadScript
    };

} (jQuery);

(function( $ ) {
	$.fn.processing = function( options ) {
		if (this.length > 1){
			this.each(function() { $(this).processing(options) });
			return this;
		}
		var opts = $.extend( {}, $.fn.processing.defaults, options );
		var elem = this;
		var prev_content = elem.children();
        var is_running = false;

		this.start = function( options ) {
			prev_content.detach();
			elem.html(opts.text)
				.append(opts.icon);
            is_running = true;
			return elem;
		};
		this.stop = function( content ) {
			if(typeof content !== 'undefined'){
				prev_content.remove();
				elem.html(content);
			} else {
				elem.html('').append(prev_content);
			}
            is_running = false;
			return elem;
		};
        this.isProcessing = function(){
            return is_running;
        };

		return this;
	};
	$.fn.processing.defaults = {
		icon: '<i class="fa fa-cog fa-spin" style="margin-left:5px;"></i>',
		text: 'processing'
	};
})( jQuery );