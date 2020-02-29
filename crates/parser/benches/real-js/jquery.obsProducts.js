obs.products = (function ($) {

    var config = {
        
    };

    var _initialize = function (options) {
        if (options) {
            obs.utils.merge(config, options);
        }
    };

    var _getProductOptions = function (productId, includeInactive, includeOnlyEnablableInactive, includeDownloadableOnly) {
        var url = "/api/products/options/" + productId + "/0/" + includeInactive + "/" + includeOnlyEnablableInactive;

        if (includeDownloadableOnly) {
            url = "" + url + "?DownloadableOnly=" + includeDownloadableOnly;
        }

        return $.ajax({
            type: 'GET',
            url: url,
            dataType: "json",
            error: function (XMLHttpRequest, textStatus, errorThrown) {
                //$.uiDialog(errorThrown, 'Error', 'Warning');
            }
        });
    };
	
	var getProductOptions = function (target, productId, includeInactive, includeOnlyEnablableInactive, excludePrintOptions, callback) {
        if (productId > 0) {
            if (typeof includeInactive === "undefined" || includeInactive === null) {
                includeInactive = 1;
            } else {
                includeInactive = Number(includeInactive);
            }
            if (typeof includeInactive === "undefined" || includeInactive === null) {
                includeOnlyEnablableInactive = 0;
            } else {
                includeOnlyEnablableInactive = Number(includeOnlyEnablableInactive);
            }
            if (typeof excludePrintOptions === "undefined" || excludePrintOptions === null) {
                excludePrintOptions = 0;
            } else {
                excludePrintOptions = Number(excludePrintOptions);
            }

            var xhr = _getProductOptions(productId, includeInactive, includeOnlyEnablableInactive, excludePrintOptions);

            xhr.success(function (data) {
                console.info(data.message);
                obs.utils.loadSelectOptions(target, data.message);
				if (typeof callback === "function") {
					// Call it, since we have confirmed it is callable​
					callback();
				}
            });
        } else {
            obs.utils.loadSelectOptions(target, obs.utils.defaultOptionValue());
        }
    };

    return {
        initialize: _initialize,
        getProductOptions: getProductOptions
    };

})(jQuery);