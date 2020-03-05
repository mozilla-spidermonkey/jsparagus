var WebBlender = function($) {
    var messageHandlers = [];
    var timeoutMilliseconds = 30000;
    var loadingTimeout;
    var preloading = false;
    var loaded = false;

    /**
        Processes post messages.
        @private
      */
    var processMessage = function(event) {
        var eventToProcess = event.originalEvent || event;
        if (eventToProcess && eventToProcess.data) {
            var parsed;
            try {
                parsed = JSON.parse(eventToProcess.data);
                if (parsed.message === 'ready') {
                    clearTimeout(loadingTimeout);
                }
            } catch (e) {
                // failed to parse message
            }
        }

        for (var i = 0; i < messageHandlers.length; i++) {
            messageHandlers[i](eventToProcess);
        }
    };

    $(window).on('message', processMessage);

    var parentElement;

    var getCliBlendUrl = function(environment, client, deviceFamily) {
        var env = environment && environment.toLowerCase && environment.toLowerCase() || '';
        var host;
        switch (env) {
            case 'dev':
                host = 'cliblends.dev.microsoft.com';
                break;
            case 'int':
                host = 'uniblends.www.microsoft.com/int';
                break;
            case 'ppe':
                host = 'uniblends.www.microsoft.com/ppe';
                break;
            case 'prod':
            default:
                host = 'www.microsoft.com/uniblends';
                break;
        }

        var url = 'https://{host}/{client}{deviceFamily}'
            .replace('{host}', host)
            .replace('{client}', client && $.trim(client) ? '?client=' + client : '?client=')
            .replace('{deviceFamily}', deviceFamily && $.trim(deviceFamily) ? '&deviceFamily=' + $.trim(deviceFamily) : '');

        return url;
    };

    var getConfigUrl = function(environment, client) {
        var env = environment && environment.toLowerCase && environment.toLowerCase() || '';
        var host;
        switch (env) {
            case 'dev':
                host = 'oneblend.dev.microsoft.com';
                break;
            case 'int':
                host = 'unistoreblends-int.www.microsoft.com';
                break;
            case 'ppe':
                host = 'unistoreblends-ppe.www.microsoft.com';
                break;
            //case 'prod':
            default:
                host = 'www.microsoft.com';
                break;
        }
        var url = 'https://{host}/webblend/api/config{client}'
            .replace('{host}', host)
            .replace('{client}', client && $.trim(client) ? '?client=' + client : '');

        return url;
    };

    var getPurchaseUrl = function(environment, purchaseServiceVersion) {
        var env = environment && environment.toLowerCase && environment.toLowerCase() || '';
        var host;
        switch (env) {
            case 'dev':
            case 'int':
                host = 'purchase-int.mp.microsoft.com';
                break;
            default:
                host = 'purchase.mp.microsoft.com';
                break;
        }
        var url = 'https://{host}/{purchaseServiceVersion}/users/me/orders'
            .replace('{host}', host)
            .replace('{purchaseServiceVersion}', purchaseServiceVersion == 7 ? 'v7.0' : 'v6.0');

        return url;
    };

    var createGuid = function() {
        var guidPattern = 'xxxxxxxx-xxxx-4xxx-Rxxx-xxxxxxxxxxxx';

        function randomHexDigit() {
            return Math.floor(Math.random() * 16).toString(16);
        }

        var result = guidPattern.replace(/x/g, randomHexDigit);

        // update remainig 2 bits of fist digit of the clock_seq_hi_and_reserved:
        return result.replace('R', (8 | Math.floor(Math.random() * 3)).toString(16));
    };

    /**
        Generates the iframe that will host the blend.
        @private
      */
    var ensureBlendFrame = function(frameContainer, iframeOptions) {
        var frameId = 'wb_auto_blend_container';
        var frame = $('#' + frameId);
        var width = iframeOptions && iframeOptions.width || '456px';
        var height = iframeOptions && iframeOptions.height || '560px';
        var title = iframeOptions && iframeOptions.title || '';

        if (frame.length) {
            frame.css({
                height: height,
                width: width
            });
        } else {
            frame = $('<iframe />', {
                id: frameId,
                name: frameId,
                src: '',
                title: title,
                style: 'width:{width}; height:{height}; position:relative; top:0; left:0; border:0; outline:none; display:block'.replace('{width}', width).replace('{height}', height)
            });

            frame.appendTo(frameContainer);
        }
        return frame;
    };

    /**
        Checks for flight.
        @private
      */
    var isFlightEnabled = function(flightSetByClient, flightToVerify) {
        var isEnabled = (document && document.cookie && document.cookie.indexOf(flightToVerify) > -1);
        if (!isEnabled) {
            var flights = flightSetByClient && flightSetByClient.split(',') || [];
            for (var i = 0; !isEnabled && i < flights.length; i++) {
                if (flights[i] === flightToVerify) {
                    isEnabled = true;
                }
            }
        }
        return isEnabled;
    };

    /**
        sends startData to the blend as a postMessage.
        @private
    */
    var postStartData = function(frameId, postBody) {
        // for some reason jQuery selector and getElementById are not returning the same object back so we need to use getElementById here.
        var blendFrameInDom = document.getElementById(frameId);
        blendFrameInDom.contentWindow.postMessage(JSON.stringify({ startdata: postBody }), '*');
    };

    /**
        Loads the blend.
        @private
    */
    var openBlend = function(postBody, parentElementId, environment, clientType, route, iframeOptions, deviceFamily) {
        parentElement = $('#' + parentElementId);

        var blendFrame = ensureBlendFrame(parentElement, iframeOptions);

        var start = Date.now();
        if (!preloading || blendFrame.attr('src') === '') {
            // Reset the frame so we completely reload
            loaded = false;
            blendFrame.attr('src', 'about:blank');

            var iFrameSrc = getCliBlendUrl(environment, clientType, deviceFamily);
            blendFrame.attr('src', iFrameSrc);
        } else {
            postBody.preloadedOpenTime = start;
            preloading = false;
        }

        var onload = function() {
            loaded = true;

            // This timing deliberately doesn't count preloading time, so that the caller sees the benefit of preloading
            var iframeLoadedTime = Date.now();
            var iframeLoadedSpan = iframeLoadedTime - start;

            window.postMessage(JSON.stringify({
                message: 'status',
                data: 'blenderSDK: Loaded in ' + iframeLoadedSpan + 'ms'
            }), '*');

            postBody.Client = clientType;
            postBody.configUrl = getConfigUrl(environment, clientType);
            postBody.selApps = route;

            postStartData(blendFrame.attr('id'), postBody);
        };

        if (!loaded) {
            blendFrame.one('load', onload);
        } else {
            onload();
        }
    };

    /**
        Updates post body for style.
        @private
      */
    var updateStyle = function(postBody, style) {
        if (style) {
            postBody.Layout = style.layout || '';
            postBody.CssOverride = style.cssOverride || '';
            postBody.Border = style.border || '';
            postBody.Brand = style.brand || '';
        }
    };

    /**
       PreLoads the Html for the blend into a div child of the provided element.
       @public
       @method preLoadBlend
       @param {inputData} it is an object with all input properties
         inputData = {
             ParentElementId: ParentElementId,
             Environment: Environment,
             ClientType: ClientType,
             DeviceFamily: deviceFamily,
         }
       {Environment} see resolveEnvironment for options (required)
       {ClientType} see clientTypes (required)
       {DeviceFamily} deviceFamily in which Blends will be loaded. (e.g Windows.Xbox or Windows.Desktop) see the wiki for more details (optional)
       {ParentElementId} DOM element (e.g. div, span) into which Blends will be loaded (required)
     */

    var preLoadBlend = function(inputData) {
        if (inputData) {
            parentElement = $('#' + inputData.ParentElementId);

            var blendFrame = ensureBlendFrame(parentElement, { height: 0, width: 0, title: '' });

            loaded = false;
            preloading = true;
            blendFrame.attr('src', 'about:blank');

            var iFrameSrc = getCliBlendUrl(inputData.Environment, inputData.ClientType, inputData.DeviceFamily);
            blendFrame.attr('src', iFrameSrc);

            blendFrame.one('load', function() {
                loaded = true;
            });

            window.postMessage(JSON.stringify({
                message: 'status',
                data: 'blenderSDK: Preload started'
            }), '*');
        }
    };

    /**
        Loads the Html for the purchase blend into a div child of the provided element.
        @public
        @method purchaseSingleItem
        @param {inputData} it is an object with all input properties
          inputData = {
              AvailabilityId: AvailabilityId,
              ProductId: ProductId,
              SkuId: SkuId,
              Options: Options,
              Auth: Auth,
              XToken: XToken,
              ParentElementId: ParentElementId,
              Environment: Environment,
              Flight: Flight,
              ClientType: ClientType,
              Culture: Culture,
              Market: Market,
              CV: CV,
              IdentityType: IdentityType,
              IdentityValue: IdentityValue,
              MediaOptions: MediaOptions,
              IframeOptions: IframeOptions,
              PurchaseServiceVersion: PurchaseServiceVersion,
              Style: Style,
              Quantity: Quantity,
              DeviceFamily: deviceFamily,
              TestClient: TestClient,
              UseDelegatedAuth: UseDelegatedAuth,
              OrderId: OrderId,
              Order: Order,
              Jwt: Jwt,
              Product: Product,
              BeneficiaryData: BeneficiaryData,
              PurchaseScenario: 'GiftPurchase'
          }
        {AvailabilityId} availability id of product to add to new order (required)
        {ProductId} product id of product to add to new order (required)
        {SkuId} sku id of product to add to new order (required)
        {Options} campaign options (optional)
        {Auth } RPS compact ticket (either Auth or XToken is required)
        {XToken} xToken (either Auth or XToken is required)
        {ParentElementId} DOM element (e.g. div, span) into which Blends will be loaded (required)
        {Environment} see resolveEnvironment for options (required)
        {Flight} see wiki for options (optional)
        {ClientType} see clientTypes (required)
        {Culture} UI culture (required)
        {Market} service market (required)
        {CV} correlation-vector (required)
        {IdentityType} auth identityType (optional)
        {IdentityValue} auth identityValue (optional)
        {MediaOptions} media-specific options (optional)
        {iframeOptions} object specifying desired width, height, and title overrides of iframe (optional)
        {PurchaseServiceVersion} string specifying desired version of purchase service to use. Will default if not supplied to oldest supported version (optional)
        {Style} layout, css override, border, and brand values (optional)
        {Quantity} quantity of the item to add to new order (optional)
        {DeviceFamily} deviceFamily in which Blends will be loaded. (e.g Windows.Xbox or Windows.Desktop) see the wiki for more details (optional)
        {TestClient} only passed if its Quality Store pretending to be a client (e.g QualityStore ) (optional)
        {UseDelegatedAuth} only passed true if the site needs to get the delegated auth token instead of authCT. (e.g true ) (optional)
        {OrderId} order id of the order, if the order was already created (optional)
        {Order} the order blob retrieved from M$ Purchase from get or create order (optional)
        {Jwt} Family Jwt for beneficiary relationship with purchaser (optional)
        {Product} the product blob retrieved from M$ Catalog that contains the PSA for the order (optional)
        {BeneficiaryData} for beneficiary msa information , includes First name, last name and msa email address (optional)
        {PurchaseScenario: 'GiftPurchase'} Set this property when the 'Buy as Gift' is clicked to invoke gift flow (optional)
        {MessageChannel} for specifying the messaging channel (postmessage/notify). If not specified, both will be used. (optional)
        {DisplayCompletion} indicates if a confirmation page should be displayed post-purchase (optional)
      */
    var purchaseSingleItem = function(inputData) {
        var postBody = {};
        if (inputData) {
            postBody.AvailabilityId = inputData.AvailabilityId || '';
            postBody.ProductId = inputData.ProductId || '';
            postBody.SkuId = inputData.SkuId || '';
            postBody.Quantity = inputData.Quantity || 1;
            postBody.Auth = inputData.Auth || '';
            postBody.XToken = inputData.XToken || '';
            postBody.Culture = inputData.Culture || '';
            postBody.Market = inputData.Market || '';
            postBody.CV = inputData.CV || '';
            postBody.Flights = inputData.Flight || '';
            postBody.IdentityType = inputData.IdentityType;
            postBody.IdentityValue = inputData.IdentityValue;
            postBody.PurchaseServiceVersion = inputData.PurchaseServiceVersion || '';
            postBody.useDelegatedAuth = inputData.UseDelegatedAuth || false;
            postBody.MessageChannel = inputData.MessageChannel || '';
            postBody.DisplayCompletion = !!inputData.DisplayCompletion;

            // Explicit not setting default value here, if these value aren't passed in,
            // we would like them to be undefined
            postBody.OrderId = inputData.OrderId;
            postBody.Order = inputData.Order;
            postBody.Jwt = inputData.Jwt;
            postBody.Product = inputData.Product;

            // this will only be used for filtering BI reports
            postBody.TestClient = inputData.TestClient || '';

            // For WebStore, all exclusivity parameters should be set to PUBLIC
            postBody.DeviceContext = 'moId=PUBLIC&oemId=PUBLIC&scmId=PUBLIC';

            if (inputData.Options) {
                postBody.CallerApplicationId = inputData.Options.CallerApplicationId || '';
                postBody.UniversalAffiliateId = inputData.Options.UniversalAffiliateId || '';
                postBody.CampaignId = inputData.Options.CampaignId || '';
                postBody.DevOfferId = inputData.Options.DevOfferId || '';
                postBody.OptionalCampaignId = inputData.Options.OptionalCampaignId || '';
                postBody.OfferType = inputData.Options.OfferType || '';
                postBody.AddPiSuccessUrl = inputData.Options.AddPiSuccessUrl || '';
                postBody.AddPiFailureUrl = inputData.Options.AddPiFailureUrl || '';
            }

            if (inputData.MediaOptions) {
                postBody.ProductType = inputData.MediaOptions.ProductType || '';
                postBody.TransactionType = inputData.MediaOptions.TransactionType || 'buy';
                postBody.Title = inputData.MediaOptions.Title || '';
                postBody.TitleNo = inputData.MediaOptions.TitleNo || '';
                postBody.SubTitle1 = inputData.MediaOptions.SubTitle1 || '';
                postBody.SubTitle2 = inputData.MediaOptions.SubTitle2 || '';
                postBody.ExpirationBeforePlayInHours = inputData.MediaOptions.ExpirationBeforePlayInHours || '';
                postBody.ExpirationAfterPlayInHours = inputData.MediaOptions.ExpirationAfterPlayInHours || '';
            }

            if (inputData.BeneficiaryData) {
                postBody.BeneficiaryEmail = inputData.BeneficiaryData.email;
                postBody.BeneficiaryFirstName = inputData.BeneficiaryData.firstName || '';
                postBody.BeneficiaryLastName = inputData.BeneficiaryData.lastName || '';
            }

            var flowToInvoke = 'purchase.confirm';
            if (inputData.PurchaseScenario === 'GiftPurchase') {
                flowToInvoke = 'purchase.gift';
            }

            updateStyle(postBody, inputData.Style);

            openBlend(postBody, inputData.ParentElementId, inputData.Environment, inputData.ClientType, flowToInvoke, inputData.IframeOptions, inputData.DeviceFamily || '');
        }
    };

    /**
        Purchases the free item silently and messages the result. If it fails, loads the Html for the purchase blend into a div child of the provided element in order to show the error.
        @public
        @method purchaseFreeItem
        @param {inputData} it is an object with all input properties
        inputData = {
            AvailabilityId: AvailabilityId,
            ProductId: ProductId,
            SkuId: SkuId,
            Options: Options,
            Auth: Auth,
            XToken: XToken,
            ParentElementId: ParentElementId,
            Environment: Environment,
            Flight: Flight,
            ClientType: ClientType,
            Culture: Culture,
            Market: Market,
            CV: CV,
            IdentityType: IdentityType,
            IdentityValue: IdentityValue,
            MediaOptions: MediaOptions,
            IframeOptions: IframeOptions,
            PurchaseServiceVersion: PurchaseServiceVersion,
            Style: Style,
            Quantity: Quantity,
            DeviceFamily: deviceFamily,
            TestClient: TestClient,
            UseDelegatedAuth: UseDelegatedAuth
        }
        {AvailabilityId} availability id of product to add to new order (required)
        {ProductId} product id of product to add to new order (required)
        {SkuId} sku id of product to add to new order (required)
        {DevOfferId} DevOfferId used for affiliate tracking (optional)
        {Options} campaign options (optional)
        {Auth} RPS compact ticket (either Auth or XToken is required)
        {XToken} xToken (either Auth or XToken is required)
        {ParentElementId} DOM element (e.g. div, span) into which Blends will be loaded (required)
        {Environment} see resolveEnvironment for options (required)
        {Flight} see wiki for options (optional)
        {ClientType} see clientTypes (required)
        {Culture} UI culture (required)
        {Market} service market (required)
        {CV} correlation-vector (required)
        {IdentityType} auth identityType (optional)
        {IdentityValue} auth identityValue (optional)
        {MediaOptions} media-specific options (optional)
        {iframeOptions} object specifying desired width, height, and title overrides of iframe (optional)
        {PurchaseServiceVersion} string specifying desired version of purchase service to use. Will default if not supplied to oldest supported version (optional)
        {Style} layout, css override, border, and brand values (optional)
        {Quantity} quantity of the item to add to new order (optional)
        {DeviceFamily} deviceFamily in which Blends will be loaded. (e.g Windows.Xbox or Windows.Desktop) see the wiki for more details (optional)
        {TestClient} only passed if its Quality Store pretending to be a client (e.g QualityStore ) (optional)
        {UseDelegatedAuth} only passed true if the site needs to get the delegated auth token instead of authCT. (e.g true ) (optional)
        {MessageChannel} for specifying the messaging channel (postmessage/notify). If not specified, both will be used. (optional)        
        {DisplayCompletion} indicates if a confirmation page should be displayed post-purchase. Only applies if UX has to be displayed (optional)
    */
    var purchaseFreeItem = function(inputData) {
        var orderAdditionalMetadata = {};
        var attachOrderAdditionalMetadata = false;

        if (inputData.Options && inputData.Options.CallerApplicationId) {
            orderAdditionalMetadata.callerApplicationId = inputData.Options.CallerApplicationId;
            attachOrderAdditionalMetadata = true;
        }

        if (inputData.Options && inputData.Options.UniversalAffiliateId) {
            orderAdditionalMetadata.universalAffiliateId = inputData.Options.UniversalAffiliateId;
            attachOrderAdditionalMetadata = true;
        }

        // Create order data with orderState=purchased
        var data = {
            billingInformation: null,
            clientContext: {
                client: inputData.ClientType,
                deviceId: inputData.DeviceId,
                deviceType: inputData.DeviceFamily,
            },
            friendlyName: null,
            orderAdditionalMetadata: attachOrderAdditionalMetadata ? JSON.stringify(orderAdditionalMetadata) : null,
            items: [{
                availabilityId: inputData.AvailabilityId,
                beneficiary: (inputData.IdentityType && inputData.IdentityValue) ? {
                    identityType: inputData.IdentityType,
                    identityValue: inputData.IdentityValue
                } : null,
                campaignId: inputData.Options.CampaignId,
                devOfferId: inputData.Options.DevOfferId,
                giftingInformation: null,
                optionalCampaignId: inputData.Options.OptionalCampaignId,
                orderManagementPolityId: null,
                productId: inputData.ProductId,
                quantity: inputData.Quantity,
                renewalContext: null,
                setBlockInformation: null,
                skuId: inputData.SkuId
            }],
            language: inputData.Languages || inputData.Culture,
            market: inputData.Market,
            orderId: createGuid(),
            orderState: 'Purchased'
        };

        // Call purchase 'create order' API
        $.ajax({
            type: 'POST',
            url: getPurchaseUrl(inputData.Environment, inputData.PurchaseServiceVersion),
            data: JSON.stringify(data),
            contentType: 'application/json',
            beforeSend: function(xhr) {
                xhr.setRequestHeader('MS-CV', inputData.CV);
                xhr.setRequestHeader('Authorization', inputData.Auth ? 'WLID1.0=\"' + inputData.Auth + '\"' : inputData.XToken);
            }
        }).then(function(orderData) {
            var orderLineItems = orderData && $.isArray(orderData.orderLineItems) && orderData.orderLineItems || [];
            var orderLineItem = orderLineItems[0];
            var purchaseApprovalState = orderLineItem.purchaseRestriction && orderLineItem.purchaseRestriction.purchaseApprovalState && orderLineItem.purchaseRestriction.purchaseApprovalState.toLowerCase && orderLineItem.purchaseRestriction.purchaseApprovalState.toLowerCase();
            var parentalApprovalState = orderLineItem.parentalApprovalState && orderLineItem.parentalApprovalState.toLowerCase && orderLineItem.parentalApprovalState.toLowerCase();

            // If order creation succeeded and no there's no restriction, send back a done message.
            // Else, call loadOrderPurchaseHtml with the orderId so webblends can display the error message.
            if (purchaseApprovalState === 'blockedbypolicy' ||
                purchaseApprovalState === 'blockedbyage' ||
                parentalApprovalState === 'blockedbypolicy' ||
                parentalApprovalState === 'blockedbyage') {
                loadOrderPurchaseHtml(
                    orderData.orderId,
                    inputData.Auth,
                    inputData.ParentElementId,
                    inputData.Environment,
                    inputData.Flight,
                    inputData.ClientType,
                    inputData.Culture,
                    inputData.Market,
                    inputData.CV,
                    inputData.IframeOptions,
                    inputData.PurchaseServiceVersion,
                    inputData.Jwt,
                    inputData.BeneficiaryData,
                    inputData.Style,
                    inputData.DeviceFamily,
                    inputData.TestClient,
                    inputData.MessageChannel,
                    inputData.DisplayCompletion
                );
            } else {
                window.postMessage(JSON.stringify({
                    flowId: inputData.FlowId || createGuid(),
                    message: 'done',
                    status: 'success',
                    data: {
                        orderId: orderData.orderId,
                        count: orderLineItems.length,
                        lineItems: orderLineItems
                    }
                }), '*');
            }
        }, function(error) {
            var errorData = error && error.responseJSON;
            var code = errorData && errorData.code && errorData.code.toLowerCase && errorData.code.toLowerCase();

            // If purchase returned with error with type=forbidden, we can call webblends with the orderId. An example of this is MaturityRatingsDenialUserInFamily.
            // However, if we received any other error, like 404-not found, call webblends using purchaseSingleItem.
            if (code === 'forbidden') {
                loadOrderPurchaseHtml(
                    data.orderId,
                    inputData.Auth,
                    inputData.ParentElementId,
                    inputData.Environment,
                    inputData.Flight,
                    inputData.ClientType,
                    inputData.Culture,
                    inputData.Market,
                    inputData.CV,
                    inputData.IframeOptions,
                    inputData.PurchaseServiceVersion,
                    inputData.Jwt,
                    inputData.BeneficiaryData,
                    inputData.Style,
                    inputData.DeviceFamily,
                    inputData.TestClient,
                    inputData.MessageChannel,
                    inputData.DisplayCompletion
                );
            } else {
                purchaseSingleItem(inputData);
            }
        });
    };

    /** This API will be deprecated soon. Use purchaseSingleItem.
        Loads the Html for the purchase blend into a div child of the provided element.
        @public
        @method loadSingleItemPurchaseHtml
        @for WebBlender
        @param {availabilityId} availability id of product to add to new order (required)
        @param {productId} product id of product to add to new order (required)
        @param {skuId} sku id of product to add to new order (required)
        @param {options} campaign options (optional)
        @param {auth} authentication (required)
        @param {parentElementId} DOM element (e.g. div, span) into which Blends will be loaded (required)
        @param {environment} see resolveEnvironment for options (required)
        @param {flight} see wiki for options (optional)
        @param {clientType} see clientTypes (required)
        @param {culture} UI culture (required)
        @param {market} service market (required)
        @param {cv} correlation-vector (required)
        @param {identityType} auth identityType (optional)
        @param {identityValue} auth identityValue (optional)
        @param {mediaOptions} media-specific options (optional)
        @param {iframeOptions} object specifying desired width, height, and title overrides of iframe (optional)
        @param {purchaseServiceVersion} string specifying desired version of purchase service to use. Will default if not supplied to oldest supported version (optional)
        @param {style} layout, css override, border, and brand values (optional)
        @param {quantity} quantity of the item to add to new order (optional)
        @param {deviceFamily} deviceFamily in which Blends will be loaded. (e.g Windows.Xbox or Windows.Desktop) see the wiki for more details (optional)
        @param {testClient} only passed if its Quality Store pretending to be a client (e.g QualityStore ) (optional)
        @param {messageChannel} for specifying the messaging channel (postmessage/notify). If not specified, both will be used. (optional)
        @param {displayCompletion} indicates if a confirmation page should be displayed post-purchase (optional)
      */
    var loadSingleItemPurchaseHtml = function(availabilityId, productId, skuId, options, auth, parentElementId, environment, flight, clientType, culture, market, cv, identityType, identityValue, mediaOptions, iframeOptions, purchaseServiceVersion, style, quantity, deviceFamily, testClient, messageChannel, displayCompletion) {
        var postBody = {
            AvailabilityId: availabilityId,
            ProductId: productId,
            SkuId: skuId,
            Quantity: quantity || 1,
            Auth: auth,
            Culture: culture,
            Flights: flight,
            Market: market,
            CV: cv,
            IdentityType: identityType,
            IdentityValue: identityValue,
            PurchaseServiceVersion: purchaseServiceVersion,
            TestClient: testClient || '', // this will only be used for filtering BI reports
            // For WebStore, all exclusivity parameters should be set to PUBLIC
            DeviceContext: 'moId=PUBLIC&oemId=PUBLIC&scmId=PUBLIC',
            MessageChannel: messageChannel,
            DisplayCompletion: displayCompletion
        };

        if (options) {
            postBody.CampaignId = options.campaignId || '';
            postBody.OptionalCampaignId = options.optionalCampaignId || '';
            postBody.OfferType = options.offerType || '';
        }

        if (mediaOptions) {
            postBody.ProductType = mediaOptions.productType || '';
            postBody.TransactionType = mediaOptions.transactionType || 'buy';
            postBody.Title = mediaOptions.title || '';
            postBody.TitleNo = mediaOptions.titleNo || '';
            postBody.SubTitle1 = mediaOptions.subTitle1 || '';
            postBody.SubTitle2 = mediaOptions.subTitle2 || '';
            postBody.ExpirationBeforePlayInHours = mediaOptions.expirationBeforePlayInHours || '';
            postBody.ExpirationAfterPlayInHours = mediaOptions.expirationAfterPlayInHours || '';
        }

        updateStyle(postBody, style);

        openBlend(postBody, parentElementId, environment, clientType, 'purchase.confirm', iframeOptions, deviceFamily || '');
    };

    /**
        Loads the Html for the purchase blend into a div child of the provided element.
        @public
        @method loadOrderPurchaseHtml
        @for WebBlender
        @param {orderId} order id (required)
        @param {auth} authentication (required)
        @param {parentElementId} DOM element (e.g. div, span) into which Blends will be loaded (required)
        @param {environment} see resolveEnvironment for options (required)
        @param {flight} see wiki for options (optional)
        @param {clientType} see clientTypes (required)
        @param {culture} UI culture (required)
        @param {market} service market (required)
        @param {cv} correlation-vector (required)
        @param {iframeOptions} object specifying desired width, height, and title overrides of iframe (optional)
        @param {purchaseServiceVersion} string specifying desired version of purchase service to use. Will default if not supplied to oldest supported version (optional)
        @param {jwt} token for beneficiary relationship with purchaser (optional)
        @param {beneficiaryData} data for beneficiary msa information , includes First name, last name and msa email address (optional)
        @param {style} layout, css override, border, and brand values (optional)
        @param {deviceFamily} deviceFamily in which Blends will be loaded. (e.g Windows.Xbox or Windows.Desktop) see the wiki for more details (optional)
        @param {testClient} only passed if its Quality Store pretending to be a client (e.g QualityStore ) (optional)
        @param {messageChannel} for specifying the messaging channel (postmessage/notify). If not specified, both will be used. (optional)
        @param {displayCompletion} indicates if a confirmation page should be displayed post-purchase (optional)
      */
    var loadOrderPurchaseHtml = function(orderId, auth, parentElementId, environment, flight, clientType, culture, market, cv, iframeOptions, purchaseServiceVersion, jwt, beneficiaryData, style, deviceFamily, testClient, messageChannel, displayCompletion) {
        var postBody = {
            OrderId: orderId,
            Auth: auth,
            Culture: culture,
            Flights: flight,
            Market: market,
            CV: cv,
            PurchaseServiceVersion: purchaseServiceVersion,
            Jwt: jwt,
            // For WebStore, all exclusivity parameters should be set to PUBLIC
            TestClient: testClient || '', // this will only be used for filtering BI reports
            DeviceContext: 'moId=PUBLIC&oemId=PUBLIC&scmId=PUBLIC',
            MessageChannel: messageChannel,
            DisplayCompletion: displayCompletion
        };
        if (beneficiaryData) {
            postBody.BeneficiaryEmail = beneficiaryData.email;
            postBody.BeneficiaryFirstName = beneficiaryData.firstName || '';
            postBody.BeneficiaryLastName = beneficiaryData.lastName || '';
        }

        updateStyle(postBody, style);

        openBlend(postBody, parentElementId, environment, clientType, 'purchase.confirm', iframeOptions, deviceFamily || '');
    };


    /** This API will be deprecated soon. Use addPaymentInstrument.
      Loads the Html for the Add PI blend into a div child of the provided element.
      @public
      @method loadAddPaymentInstrumentHtml
      @for WebBlender
      @param {auth} authentication (required)
      @param {parentElementId} DOM element (e.g. div, span) into which Blends will be loaded (required)
      @param {environment} see resolveEnvironment for options (required)
      @param {flight} see wiki for options (optional)
      @param {clientType} see clientTypes (required)
      @param {culture} UI culture (required)
      @param {market} service market (required)
      @param {cv} correlation-vector (required)
      @param {iframeOptions} object specifying desired width, height, and title overrides of iframe (optional)
      @param {style} layout, css override, border, and brand values (optional)
      @param {messageChannel} for specifying the messaging channel (postmessage/notify). If not specified, both will be used. (optional)
*/
    var loadAddPaymentInstrumentHtml = function(auth, parentElementId, environment, flight, clientType, culture, market, cv, iframeOptions, style, messageChannel) {
        var postBody = {
            Auth: auth,
            Culture: culture,
            Flights: flight,
            Market: market,
            CV: cv,
            context: 'primaryPi',
            MessageChannel: messageChannel,
        };

        updateStyle(postBody, style);

        openBlend(postBody, parentElementId, environment, clientType, 'paymentAndBilling.choosePaymentMethodFamily', iframeOptions);
    };


    /** This API will be deprecated soon, use editPaymentInstrument.
        Loads the Html for the edit payment instrument blend into a div child of the provided element.
        @public
        @method loadEditPaymentInstrumentHtml
        @for WebBlender
        @param {auth} authentication (required)
        @param {parentElementId} DOM element (e.g. div, span) into which Blends will be loaded (required)
        @param {environment} see resolveEnvironment for options (required)
        @param {flight} see wiki for options (optional)
        @param {clientType} see clientTypes (required)
        @param {culture} UI culture (required)
        @param {market} service market (required)
        @param {cv} correlation-vector (required)
        @param {paymentInstrumentId} payment instrument id (required)
        @param {iframeOptions} object specifying desired width, height, and title overrides of iframe (optional)
        @param {style} layout, css override, border, and brand values (optional)
        @param {messageChannel} for specifying the messaging channel (postmessage/notify). If not specified, both will be used. (optional)
      */
    var loadEditPaymentInstrumentHtml = function(auth, parentElementId, environment, flight, clientType, culture, market, cv, paymentInstrumentId, iframeOptions, style, messageChannel) {
        var postBody = {
            Auth: auth,
            Culture: culture,
            Flights: flight,
            Market: market,
            CV: cv,
            PaymentInstrumentId: paymentInstrumentId,
            MessageChannel: messageChannel,
        };

        updateStyle(postBody, style);

        openBlend(postBody, parentElementId, environment, clientType, 'paymentAndBilling.editPaymentInstrument', iframeOptions);
    };

    /**
      Loads the Html for the Add PI blend into a div child of the provided element.
      @public
      @method addPaymentInstrument
      @param {inputData} it is an object with all input properties
        inputData = {
            Auth: Auth,
            XToken: XToken,
            ParentElementId: ParentElementId,
            Environment: Environment,
            Flight: Flight,
            ClientType: ClientType,
            Culture: Culture,
            Market: Market,
            CV: CV,
            IframeOptions: IframeOptions,
            Style: Style,
            DeviceFamily: DeviceFamily,
            PaymentMethodFamily: PaymentMethodFamily,
            PaymentMethodType: PaymentMethodType,
            BillableAccountId: BillableAccountId
        }
      {Auth} RPS compact ticket (either Auth or XToken is required)
      {XToken} xToken (either Auth or XToken is required)
      {ParentElementId} DOM element (e.g. div, span) into which Blends will be loaded (required)
      {Environment} see resolveEnvironment for options (required)
      {Flight} see wiki for options (optional)
      {ClientType} see clientTypes (required)
      {Culture} UI culture (required)
      {Market} service market (required)
      {CV} correlation-vector (required)
      {iframeOptions} object specifying desired width, height, and title overrides of iframe (optional)
      {Style} layout, css override, border, and brand values (optional)
      {DeviceFamily} DeviceFamily in which Blends will be loaded. see the wiki for more details (required)
      {PaymentMethodFamily} paymentMethodFamily of payment Instrument. see the wiki for more details (required)
      {PaymentMethodType} paymentMethodFamily of payment Instrument. see the wiki for more details  (required)
      {BillableAccountId} Pass BillableAccountId for legacy subscriptions (optional)
      {MessageChannel} for specifying the messaging channel (postmessage/notify). If not specified, both will be used. (optional)
*/
    var addPaymentInstrument = function(inputData) {
        var postBody = {};
        if (inputData) {
            postBody.Auth = inputData.Auth || '';
            postBody.XToken = inputData.XToken || '';
            postBody.Culture = inputData.Culture || '';
            postBody.Flights = inputData.Flight || '';
            postBody.Market = inputData.Market || '';
            postBody.CV = inputData.CV || '';
            postBody.context = 'primaryPi';
            postBody.PaymentMethodFamily = inputData.PaymentMethodFamily || '';
            postBody.PaymentMethodType = inputData.PaymentMethodType || '';
            postBody.BillableAccountId = inputData.BillableAccountId || '';
            postBody.PurchaseServiceVersion = '7';
            postBody.Gamertag = inputData.Gamertag || '';
            postBody.GamerpicUrl = inputData.GamerpicUrl || '';
            postBody.MessageChannel = inputData.MessageChannel || '';

            var deviceFamily = inputData.DeviceFamily || 'web';

            updateStyle(postBody, inputData.Style);

            openBlend(postBody, inputData.ParentElementId, inputData.Environment, inputData.ClientType, 'paymentAndBilling.choosePaymentMethodFamily', inputData.IframeOptions, deviceFamily);
        }
    };

    /**
      Loads the Html for the Edit PI blend into a div child of the provided element.
      @public
      @method editPaymentInstrument
      @param {inputData} it is an object with all input properties
        inputData = {
            Auth: Auth,
            XToken: XToken,
            PaymentInstrumentId: PaymentInstrumentId,
            ParentElementId: ParentElementId,
            Environment: Environment,
            Flight: Flight,
            ClientType: ClientType,
            Culture: Culture,
            Market: Market,
            CV: CV,
            IframeOptions: IframeOptions,
            Style: Style,
            PaymentMethodFamily: PaymentMethodFamily,
            PaymentMethodType: PaymentMethodType,
            DeviceFamily: DeviceFamily
        }
      {Auth} RPS compact ticket (either Auth or XToken is required)
      {XToken} xToken (either Auth or XToken is required)
      {PaymentInstrumentId} The ID of the payment instrument being edited. (required)
      {ParentElementId} DOM element (e.g. div, span) into which Blends will be loaded (required)
      {Environment} see resolveEnvironment for options (required)
      {Flight} see wiki for options (optional)
      {ClientType} see clientTypes (required)
      {Culture} UI culture (required)
      {Market} service market (required)
      {CV} correlation-vector (required)
      {IframeOptions} object specifying desired width, height, and title overrides of iframe (optional)
      {Style} layout, css override, border, and brand values (optional)
      {PaymentMethodFamily} paymentMethodFamily of payment Instrument. see the wiki for more details (required)
      {PaymentMethodType} paymentMethodFamily of payment Instrument. see the wiki for more details  (required)
      {DeviceFamily} deviceFamily in which Blends will be loaded. see the wiki for more details (required)
      {MessageChannel} for specifying the messaging channel (postmessage/notify). If not specified, both will be used. (optional)
*/
    var editPaymentInstrument = function(inputData) {
        var postBody = {};
        if (inputData) {
            postBody.Auth = inputData.Auth || '';
            postBody.XToken = inputData.XToken || '';
            postBody.Culture = inputData.Culture || '';
            postBody.Flights = inputData.Flight || '';
            postBody.Market = inputData.Market || '';
            postBody.CV = inputData.CV || '';
            postBody.PaymentInstrumentId = inputData.PaymentInstrumentId;
            postBody.PaymentMethodFamily = inputData.PaymentMethodFamily || '';
            postBody.PaymentMethodType = inputData.PaymentMethodType || '';
            postBody.PurchaseServiceVersion = '7';
            postBody.MessageChannel = inputData.MessageChannel || '';

            var deviceFamily = inputData.DeviceFamily || 'web';

            updateStyle(postBody, inputData.Style);

            openBlend(postBody, inputData.ParentElementId, inputData.Environment, inputData.ClientType, 'paymentAndBilling.editPaymentInstrument', inputData.IframeOptions, deviceFamily);
        }
    };

    /**
        Loads the Html for the Redeem Token blend into a div child of the provided element.
        @public
        @method redeem
        @param {inputData} it is an object with all input properties
          inputData = {
              TokenString: TokenString,
              Auth: Auth,
              XToken: XToken,
              ParentElementId: ParentElementId,
              Environment: Environment,
              Flight: Flight,
              ClientType: ClientType,
              Culture: Culture,
              Market: Market,
              CV: CV,
              IframeOptions: IframeOptions,
              Style: Style,
              DeviceFamily: DeviceFamily,
              PurchaseServiceVersion: PurchaseServiceVersion,
              UseDelegatedAuth: UseDelegatedAuth,
              UrlRef: UrlRef
          }
        {TokenString} Token string. If non-empty, blend will skip token input page, and go directly to confirm page. (optional)
        {Auth} RPS compact ticket (either Auth or XToken is required)
        {XToken} xToken (either Auth or XToken is required)
        {ParentElementId} DOM element (e.g. div, span) into which Blends will be loaded (required)
        {Environment} see resolveEnvironment for options (required)
        {Flight} see wiki for options (optional)
        {ClientType} see clientTypes (required)
        {Culture} UI culture (required)
        {Market} service market (required)
        {CV} correlation-vector (required)
        {IframeOptions} object specifying desired width, height, and title overrides of iframe (optional)
        {Style} layout, css override, border, and brand values (optional)
        {DeviceFamily} DeviceFamily in which Blends will be loaded. see the wiki for more details (required)
        {PurchaseServiceVersion} Specified the M$ version to use. Default is the current version. (e.g 6 ) (optional)
        {UseDelegatedAuth} only passed true if the site needs to get the delegated auth token instead of authCT. (e.g true ) (optional)
        {UrlRef} urlref from a redirect. Used for telemetry (optional)
        {EnableScanLink} enable ocr scan Link (optional)
        {MessageChannel} for specifying the messaging channel (postmessage/notify). If not specified, both will be used. (optional)
    */
    var redeem = function(inputData) {
        var postBody = {};
        if (inputData) {
            postBody.tokenString = inputData.TokenString || '';
            postBody.Auth = inputData.Auth || '';
            postBody.XToken = inputData.XToken || '';
            postBody.Culture = inputData.Culture || '';
            postBody.Market = inputData.Market || '';
            postBody.CV = inputData.CV || '';
            postBody.Flights = inputData.Flight || '';
            postBody.useDelegatedAuth = inputData.UseDelegatedAuth || false;
            postBody.PurchaseServiceVersion = inputData.PurchaseServiceVersion || '7';
            postBody.UrlRef = inputData.UrlRef || '';
            postBody.OcrScanAvailable = inputData.OcrScanAvailable || '';
            postBody.MessageChannel = inputData.MessageChannel || '';

            var deviceFamily = inputData.DeviceFamily || 'web';

            updateStyle(postBody, inputData.Style);

            openBlend(postBody, inputData.ParentElementId, inputData.Environment, inputData.ClientType, 'redeem.enterCode', inputData.IframeOptions, deviceFamily);
        }
    };

    /** This API will be deprecated soon, use redeem.
       Loads the Html for the Redeem Token blend into a div child of the provided element.
       @public
       @method loadRedeemHtml
       @for WebBlender
       @param {tokenString} Token string. If non-empty, blend will skip token input page, and go directly to confirm page. (optional)
       @param {auth} authentication (required)
       @param {parentElementId} DOM element (e.g. div, span) into which Blends will be loaded (required)
       @param {environment} see resolveEnvironment for options (required)
       @param {flight} see wiki for options (optional)
       @param {clientType} see clientTypes (required)
       @param {culture} UI culture (required)
       @param {market} service market (required)
       @param {cv} correlation-vector (required)
       @param {iframeOptions} object specifying desired width, height, and title overrides of iframe (optional)
       @param {style} layout, css override, border, and brand values (optional)
       @param {urlRef} urlref from a redirect. Used for telemetry (optional)
       @param {ocrScanAvailable} enable Scan Link (optional)
       @param {messageChannel} for specifying the messaging channel (postmessage/notify). If not specified, both will be used. (optional)
     */
    var loadRedeemHtml = function(tokenString, auth, parentElementId, environment, flight, clientType, culture, market, cv, iframeOptions, style, urlRef, ocrScanAvailable, messageChannel) {
        var postBody = {
            tokenString: tokenString,
            Auth: auth,
            Culture: culture,
            Flights: flight,
            Market: market,
            CV: cv,
            PurchaseServiceVersion: '7',
            UrlRef: urlRef,
            OcrScanAvailable: ocrScanAvailable,
            MessageChannel: messageChannel
        };

        updateStyle(postBody, style);

        openBlend(postBody, parentElementId, environment, clientType, 'redeem.enterCode', iframeOptions);
    };

    /**
        Subscribes to the blend post messages.
        @public
        @method registerMessageHandler
        @for WebBlender
        @param {newHandler} subscriber to add
      */
    var registerMessageHandler = function(newHandler) {
        for (var i = 0; i < messageHandlers.length; i++) {
            if ('' + messageHandlers[i] === '' + newHandler) {
                // See 5056068
                console.error('Same message handler function already registered: ' + newHandler);
                return;
            }
        }
        messageHandlers.push(newHandler);
    };

    /**
        Unsubscribes from the blend post messages.
        @public
        @method unregisterMessageHandler
        @for WebBlender
        @param {handler} subscriber to remove
      */
    var unregisterMessageHandler = function(handler) {
        for (var i = 0; i < messageHandlers.length; i++) {
            if ('' + messageHandlers[i] === '' + handler) {
                messageHandlers.pop(handler);
            }
        }
    };

    /**
        Mapping of client types.
        @public
        @object clientTypes
        @for WebBlender
      */
    var clientTypes = {
        UniversalWebStore: 'UniversalWebStore',
        MusicVideoReading: 'MusicVideoReading',
        EnterpriseStore: 'EnterpriseStore',
        AccountMicrosoftCom: 'AccountMicrosoftCom'
    };

    return {
        loadSingleItemPurchaseHtml: loadSingleItemPurchaseHtml,
        loadOrderPurchaseHtml: loadOrderPurchaseHtml,
        loadRedeemHtml: loadRedeemHtml,
        loadAddPaymentInstrumentHtml: loadAddPaymentInstrumentHtml,
        loadEditPaymentInstrumentHtml: loadEditPaymentInstrumentHtml,
        registerMessageHandler: registerMessageHandler,
        unregisterMessageHandler: unregisterMessageHandler,
        clientTypes: clientTypes,
        addPaymentInstrument: addPaymentInstrument,
        editPaymentInstrument: editPaymentInstrument,
        purchaseSingleItem: purchaseSingleItem,
        purchaseFreeItem: purchaseFreeItem,
        redeem: redeem,
        preLoadBlend: preLoadBlend
    };
} (jQuery);