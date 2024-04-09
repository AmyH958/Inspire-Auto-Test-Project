<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_An exception has occurred              _9a800d</name>
   <tag></tag>
   <elementGuidId>48ffb9f7-a4c3-4cef-9711-8a81d67b3ff9</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='exception']/div/div[2]</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>div.exceptionDialog</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>75438681-c07f-4e41-bb19-640f57501ba2</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>exceptionDialog</value>
      <webElementGuid>71118f53-c155-4f7d-8dba-bd4035802f08</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>


  
    
  
    
      
        
          An exception has occurred!
        
        
          
            請重新登入系統!
          
        
      
      
      
      
        

    [ +/- ] Exception:
	   Unable to update OGNL expression '&lt;parsed OGNL expression>' of $AdminInventoryRule_143@3c1[AdminInventoryRule] to 1234567890abc: For input string: &quot;1234567890abc&quot;	
	
		
			org.apache.tapestry.BindingException
		
		
		
			binding:
			ExpressionBinding[AdminInventoryRule editRule.inventoryDigits]

		
			location:
			context:/WEB-INF/AdminInventoryRule.html, line 154149      &lt;/tr>
150        &lt;tr>
151          &lt;td class=&quot;table1_left&quot; width = &quot;25%&quot;>&lt;span jwcid=&quot;@Insert&quot; value=&quot;ognl:getMessage('AdminInventoryRule.41837')&quot;>Card Digits&lt;/span>:&lt;/td>
152          &lt;td jwcid=&quot;DigitsField@Any&quot;>
153          &lt;span jwcid=&quot;@Insert&quot; value=&quot;ognl:requiredDigitsField&quot; renderTag=&quot;true&quot; style=&quot;color: red;&quot;>&lt;/span>
154          &lt;input jwcid=&quot;@TextField&quot; value=&quot;ognl:editRule.inventoryDigits&quot; validators=&quot;validators:required&quot; onkeyup=&quot;verifyNumber(this)&quot; size=&quot;8&quot;/>&lt;font color=&quot;red&quot;>*&lt;/font>
155          &lt;/td>
156          
157          
158        &lt;/tr>
159        &lt;tr width=&quot;100%&quot; align=&quot;right&quot;>

		

		     

 
	
	

    [ +/- ] Exception:
	   Unable to update OGNL expression '&lt;parsed OGNL expression>' of $AdminInventoryRule_143@3c1[AdminInventoryRule] to 1234567890abc: For input string: &quot;1234567890abc&quot;	
	
		
			org.apache.hivemind.ApplicationRuntimeException
		
		
		
			component:
			$AdminInventoryRule_143@3c1[AdminInventoryRule]

		
			location:
			context:/WEB-INF/AdminInventoryRule.page, line 4, column 601&lt;?xml version=&quot;1.0&quot; encoding=&quot;UTF-8&quot;?>
2&lt;!DOCTYPE page-specification PUBLIC &quot;-//Apache Software Foundation//Tapestry Specification 4.0//EN&quot; &quot;http://jakarta.apache.org/tapestry/dtd/Tapestry_4_0.dtd&quot;>
3
4&lt;page-specification class=&quot;sibimol.web.AdminInventoryRule&quot;>
5    &lt;asset name=&quot;js_utility_js&quot; path=&quot;context:/js/utility.js&quot;/>
6    &lt;asset name=&quot;images_arrow_checked_13_png&quot; path=&quot;context:/images/arrow_checked_13.png&quot;/>
7    &lt;asset name=&quot;images_arrow_unchecked_13_png&quot; path=&quot;context:/images/arrow_unchecked_13.png&quot;/>
8&lt;/page-specification>

		

		     

 
	
	

    [ +/- ] Exception:
	   For input string: &quot;1234567890abc&quot;	
	
		
			java.lang.NumberFormatException
		
		
		

		


		
			Stack Trace:
		

		
			
				
					java.lang.NumberFormatException.forInputString(NumberFormatException.java:48)
					java.lang.Long.parseLong(Long.java:419)
					java.lang.Long.parseLong(Long.java:468)
					ognl.OgnlOps.longValue(OgnlOps.java:220)
					ognl.OgnlOps.convertValue(OgnlOps.java:587)
					ognl.OgnlOps.convertValue(OgnlOps.java:509)
					$ASTChain_188ab28964d.ref1($ASTChain_188ab28964d.java)
					$ASTChain_188ab28964d.set($ASTChain_188ab28964d.java)
					org.apache.tapestry.services.impl.ExpressionEvaluatorImpl.write(ExpressionEvaluatorImpl.java:178)
					$ExpressionEvaluator_188ab288d27.write($ExpressionEvaluator_188ab288d27.java)
					org.apache.tapestry.binding.ExpressionBinding.setObject(ExpressionBinding.java:224)
					$TextField_9.setValue($TextField_9.java)
					org.apache.tapestry.form.TextField.rewindFormComponent(TextField.java:105)
					org.apache.tapestry.form.AbstractFormComponent.renderComponent(AbstractFormComponent.java:90)
					org.apache.tapestry.AbstractComponent.render(AbstractComponent.java:724)
					org.apache.tapestry.services.impl.DojoAjaxResponseBuilder.render(DojoAjaxResponseBuilder.java:635)
					org.apache.tapestry.AbstractComponent.renderBody(AbstractComponent.java:538)
					org.apache.tapestry.components.Any.renderComponent(Any.java:44)
					org.apache.tapestry.AbstractComponent.render(AbstractComponent.java:724)
					org.apache.tapestry.services.impl.DojoAjaxResponseBuilder.render(DojoAjaxResponseBuilder.java:635)
					org.apache.tapestry.AbstractComponent.renderBody(AbstractComponent.java:538)
					org.apache.tapestry.form.FormSupportImpl.rewind(FormSupportImpl.java:624)
					org.apache.tapestry.form.Form.renderComponent(Form.java:196)
					org.apache.tapestry.AbstractComponent.render(AbstractComponent.java:724)
					org.apache.tapestry.services.impl.DojoAjaxResponseBuilder.render(DojoAjaxResponseBuilder.java:635)
					org.apache.tapestry.form.Form.rewind(Form.java:269)
					org.apache.tapestry.engine.RequestCycle.rewindForm(RequestCycle.java:469)
					org.apache.tapestry.form.Form.trigger(Form.java:280)
					org.apache.tapestry.engine.DirectService.triggerComponent(DirectService.java:166)
					org.apache.tapestry.engine.DirectService.service(DirectService.java:142)
					$IEngineService_188ab288c36.service($IEngineService_188ab288c36.java)
					org.apache.tapestry.services.impl.EngineServiceOuterProxy.service(EngineServiceOuterProxy.java:72)
					org.apache.tapestry.engine.AbstractEngine.service(AbstractEngine.java:241)
					org.apache.tapestry.services.impl.InvokeEngineTerminator.service(InvokeEngineTerminator.java:54)
					$WebRequestServicer_188ab288c09.service($WebRequestServicer_188ab288c09.java)
					$WebRequestServicer_188ab288c05.service($WebRequestServicer_188ab288c05.java)
					org.apache.tapestry.services.impl.WebRequestServicerPipelineBridge.service(WebRequestServicerPipelineBridge.java:61)
					$ServletRequestServicer_188ab288beb.service($ServletRequestServicer_188ab288beb.java)
					org.apache.tapestry.request.DecodedRequestInjector.service(DecodedRequestInjector.java:55)
					$ServletRequestServicerFilter_188ab288be7.service($ServletRequestServicerFilter_188ab288be7.java)
					$ServletRequestServicer_188ab288bed.service($ServletRequestServicer_188ab288bed.java)
					org.apache.tapestry.multipart.MultipartDecoderFilter.service(MultipartDecoderFilter.java:52)
					$ServletRequestServicerFilter_188ab288be5.service($ServletRequestServicerFilter_188ab288be5.java)
					$ServletRequestServicer_188ab288bed.service($ServletRequestServicer_188ab288bed.java)
					org.apache.tapestry.services.impl.SetupRequestEncoding.service(SetupRequestEncoding.java:53)
					$ServletRequestServicerFilter_188ab288be9.service($ServletRequestServicerFilter_188ab288be9.java)
					$ServletRequestServicer_188ab288bed.service($ServletRequestServicer_188ab288bed.java)
					$ServletRequestServicer_188ab288bdf.service($ServletRequestServicer_188ab288bdf.java)
					org.apache.tapestry.ApplicationServlet.doService(ApplicationServlet.java:126)
					org.apache.tapestry.ApplicationServlet.doPost(ApplicationServlet.java:171)
					javax.servlet.http.HttpServlet.service(HttpServlet.java:643)
					javax.servlet.http.HttpServlet.service(HttpServlet.java:723)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:290)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					org.apache.tapestry.RedirectFilter.doFilter(RedirectFilter.java:103)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:235)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					org.tuckey.web.filters.urlrewrite.RuleChain.handleRewrite(RuleChain.java:176)
					org.tuckey.web.filters.urlrewrite.RuleChain.doRules(RuleChain.java:145)
					org.tuckey.web.filters.urlrewrite.UrlRewriter.processRequest(UrlRewriter.java:92)
					org.tuckey.web.filters.urlrewrite.UrlRewriteFilter.doFilter(UrlRewriteFilter.java:381)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:235)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					ime.tinread.web.servlet.filters.HTTPCacheFilter.doFilter(HTTPCacheFilter.java:26)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:235)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:359)
					org.springframework.security.intercept.web.FilterSecurityInterceptor.invoke(FilterSecurityInterceptor.java:109)
					org.springframework.security.intercept.web.FilterSecurityInterceptor.doFilter(FilterSecurityInterceptor.java:83)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.ui.SessionFixationProtectionFilter.doFilterHttp(SessionFixationProtectionFilter.java:67)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.ui.ExceptionTranslationFilter.doFilterHttp(ExceptionTranslationFilter.java:101)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.providers.anonymous.AnonymousProcessingFilter.doFilterHttp(AnonymousProcessingFilter.java:105)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.ui.rememberme.RememberMeProcessingFilter.doFilterHttp(RememberMeProcessingFilter.java:116)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.wrapper.SecurityContextHolderAwareRequestFilter.doFilterHttp(SecurityContextHolderAwareRequestFilter.java:91)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.ui.basicauth.BasicProcessingFilter.doFilterHttp(BasicProcessingFilter.java:173)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.ui.AbstractProcessingFilter.doFilterHttp(AbstractProcessingFilter.java:271)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.ui.logout.LogoutFilter.doFilterHttp(LogoutFilter.java:89)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.context.HttpSessionContextIntegrationFilter.doFilterHttp(HttpSessionContextIntegrationFilter.java:235)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.util.FilterChainProxy.doFilter(FilterChainProxy.java:174)
					org.springframework.web.filter.DelegatingFilterProxy.invokeDelegate(DelegatingFilterProxy.java:236)
					org.springframework.web.filter.DelegatingFilterProxy.doFilter(DelegatingFilterProxy.java:167)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:235)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					ime.tinread.servlets.LogbackFilter.doFilter(LogbackFilter.java:28)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:235)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					transtech.webutil.DisableUrlSessionFilter.doFilter(DisableUrlSessionFilter.java:58)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:235)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					transtech.webutil.IPAccessFilter.doFilter(IPAccessFilter.java:95)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:235)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					org.apache.catalina.core.StandardWrapperValve.invoke(StandardWrapperValve.java:233)
					org.apache.catalina.core.StandardContextValve.invoke(StandardContextValve.java:191)
					org.apache.catalina.core.StandardHostValve.invoke(StandardHostValve.java:127)
					org.apache.catalina.valves.ErrorReportValve.invoke(ErrorReportValve.java:103)
					org.apache.catalina.core.StandardEngineValve.invoke(StandardEngineValve.java:109)
					org.apache.catalina.connector.CoyoteAdapter.service(CoyoteAdapter.java:293)
					org.apache.coyote.http11.Http11Processor.process(Http11Processor.java:861)
					org.apache.coyote.http11.Http11Protocol$Http11ConnectionHandler.process(Http11Protocol.java:620)
					org.apache.tomcat.util.net.JIoEndpoint$Worker.run(JIoEndpoint.java:489)
					java.lang.Thread.run(Thread.java:662)
					
					
			
		
 
	
	
 


      
      
      
      
      
         [+/-]  請顯示訊息 
      
      
      
      
    
    
  

</value>
      <webElementGuid>8be5f554-a24e-4a9e-a61c-bea23a6ba36b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;exception&quot;)/div[1]/div[@class=&quot;exceptionDialog&quot;]</value>
      <webElementGuid>26c78651-7da4-41c8-9d8b-1f781a5aef8c</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='exception']/div/div[2]</value>
      <webElementGuid>b695b089-cea2-4203-9278-45b9be8a022d</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Close'])[1]/following::div[1]</value>
      <webElementGuid>67ab2e24-d44a-4cbb-aa16-7dc39be20d84</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)=concat('id(', '&quot;', 'exception', '&quot;', ')/div[1]/div[@class=', '&quot;', 'exceptionDialog', '&quot;', ']')])[1]/following::div[4]</value>
      <webElementGuid>8393be0f-2a9f-4c4b-bf00-1b1c9c75a46c</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[4]/div/div[2]</value>
      <webElementGuid>f2770a80-9add-4737-b63d-17679d58f57c</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;


  
    
  
    
      
        
          An exception has occurred!
        
        
          
            請重新登入系統!
          
        
      
      
      
      
        

    [ +/- ] Exception:
	   Unable to update OGNL expression &quot; , &quot;'&quot; , &quot;&lt;parsed OGNL expression>&quot; , &quot;'&quot; , &quot; of $AdminInventoryRule_143@3c1[AdminInventoryRule] to 1234567890abc: For input string: &quot;1234567890abc&quot;	
	
		
			org.apache.tapestry.BindingException
		
		
		
			binding:
			ExpressionBinding[AdminInventoryRule editRule.inventoryDigits]

		
			location:
			context:/WEB-INF/AdminInventoryRule.html, line 154149      &lt;/tr>
150        &lt;tr>
151          &lt;td class=&quot;table1_left&quot; width = &quot;25%&quot;>&lt;span jwcid=&quot;@Insert&quot; value=&quot;ognl:getMessage(&quot; , &quot;'&quot; , &quot;AdminInventoryRule.41837&quot; , &quot;'&quot; , &quot;)&quot;>Card Digits&lt;/span>:&lt;/td>
152          &lt;td jwcid=&quot;DigitsField@Any&quot;>
153          &lt;span jwcid=&quot;@Insert&quot; value=&quot;ognl:requiredDigitsField&quot; renderTag=&quot;true&quot; style=&quot;color: red;&quot;>&lt;/span>
154          &lt;input jwcid=&quot;@TextField&quot; value=&quot;ognl:editRule.inventoryDigits&quot; validators=&quot;validators:required&quot; onkeyup=&quot;verifyNumber(this)&quot; size=&quot;8&quot;/>&lt;font color=&quot;red&quot;>*&lt;/font>
155          &lt;/td>
156          
157          
158        &lt;/tr>
159        &lt;tr width=&quot;100%&quot; align=&quot;right&quot;>

		

		     

 
	
	

    [ +/- ] Exception:
	   Unable to update OGNL expression &quot; , &quot;'&quot; , &quot;&lt;parsed OGNL expression>&quot; , &quot;'&quot; , &quot; of $AdminInventoryRule_143@3c1[AdminInventoryRule] to 1234567890abc: For input string: &quot;1234567890abc&quot;	
	
		
			org.apache.hivemind.ApplicationRuntimeException
		
		
		
			component:
			$AdminInventoryRule_143@3c1[AdminInventoryRule]

		
			location:
			context:/WEB-INF/AdminInventoryRule.page, line 4, column 601&lt;?xml version=&quot;1.0&quot; encoding=&quot;UTF-8&quot;?>
2&lt;!DOCTYPE page-specification PUBLIC &quot;-//Apache Software Foundation//Tapestry Specification 4.0//EN&quot; &quot;http://jakarta.apache.org/tapestry/dtd/Tapestry_4_0.dtd&quot;>
3
4&lt;page-specification class=&quot;sibimol.web.AdminInventoryRule&quot;>
5    &lt;asset name=&quot;js_utility_js&quot; path=&quot;context:/js/utility.js&quot;/>
6    &lt;asset name=&quot;images_arrow_checked_13_png&quot; path=&quot;context:/images/arrow_checked_13.png&quot;/>
7    &lt;asset name=&quot;images_arrow_unchecked_13_png&quot; path=&quot;context:/images/arrow_unchecked_13.png&quot;/>
8&lt;/page-specification>

		

		     

 
	
	

    [ +/- ] Exception:
	   For input string: &quot;1234567890abc&quot;	
	
		
			java.lang.NumberFormatException
		
		
		

		


		
			Stack Trace:
		

		
			
				
					java.lang.NumberFormatException.forInputString(NumberFormatException.java:48)
					java.lang.Long.parseLong(Long.java:419)
					java.lang.Long.parseLong(Long.java:468)
					ognl.OgnlOps.longValue(OgnlOps.java:220)
					ognl.OgnlOps.convertValue(OgnlOps.java:587)
					ognl.OgnlOps.convertValue(OgnlOps.java:509)
					$ASTChain_188ab28964d.ref1($ASTChain_188ab28964d.java)
					$ASTChain_188ab28964d.set($ASTChain_188ab28964d.java)
					org.apache.tapestry.services.impl.ExpressionEvaluatorImpl.write(ExpressionEvaluatorImpl.java:178)
					$ExpressionEvaluator_188ab288d27.write($ExpressionEvaluator_188ab288d27.java)
					org.apache.tapestry.binding.ExpressionBinding.setObject(ExpressionBinding.java:224)
					$TextField_9.setValue($TextField_9.java)
					org.apache.tapestry.form.TextField.rewindFormComponent(TextField.java:105)
					org.apache.tapestry.form.AbstractFormComponent.renderComponent(AbstractFormComponent.java:90)
					org.apache.tapestry.AbstractComponent.render(AbstractComponent.java:724)
					org.apache.tapestry.services.impl.DojoAjaxResponseBuilder.render(DojoAjaxResponseBuilder.java:635)
					org.apache.tapestry.AbstractComponent.renderBody(AbstractComponent.java:538)
					org.apache.tapestry.components.Any.renderComponent(Any.java:44)
					org.apache.tapestry.AbstractComponent.render(AbstractComponent.java:724)
					org.apache.tapestry.services.impl.DojoAjaxResponseBuilder.render(DojoAjaxResponseBuilder.java:635)
					org.apache.tapestry.AbstractComponent.renderBody(AbstractComponent.java:538)
					org.apache.tapestry.form.FormSupportImpl.rewind(FormSupportImpl.java:624)
					org.apache.tapestry.form.Form.renderComponent(Form.java:196)
					org.apache.tapestry.AbstractComponent.render(AbstractComponent.java:724)
					org.apache.tapestry.services.impl.DojoAjaxResponseBuilder.render(DojoAjaxResponseBuilder.java:635)
					org.apache.tapestry.form.Form.rewind(Form.java:269)
					org.apache.tapestry.engine.RequestCycle.rewindForm(RequestCycle.java:469)
					org.apache.tapestry.form.Form.trigger(Form.java:280)
					org.apache.tapestry.engine.DirectService.triggerComponent(DirectService.java:166)
					org.apache.tapestry.engine.DirectService.service(DirectService.java:142)
					$IEngineService_188ab288c36.service($IEngineService_188ab288c36.java)
					org.apache.tapestry.services.impl.EngineServiceOuterProxy.service(EngineServiceOuterProxy.java:72)
					org.apache.tapestry.engine.AbstractEngine.service(AbstractEngine.java:241)
					org.apache.tapestry.services.impl.InvokeEngineTerminator.service(InvokeEngineTerminator.java:54)
					$WebRequestServicer_188ab288c09.service($WebRequestServicer_188ab288c09.java)
					$WebRequestServicer_188ab288c05.service($WebRequestServicer_188ab288c05.java)
					org.apache.tapestry.services.impl.WebRequestServicerPipelineBridge.service(WebRequestServicerPipelineBridge.java:61)
					$ServletRequestServicer_188ab288beb.service($ServletRequestServicer_188ab288beb.java)
					org.apache.tapestry.request.DecodedRequestInjector.service(DecodedRequestInjector.java:55)
					$ServletRequestServicerFilter_188ab288be7.service($ServletRequestServicerFilter_188ab288be7.java)
					$ServletRequestServicer_188ab288bed.service($ServletRequestServicer_188ab288bed.java)
					org.apache.tapestry.multipart.MultipartDecoderFilter.service(MultipartDecoderFilter.java:52)
					$ServletRequestServicerFilter_188ab288be5.service($ServletRequestServicerFilter_188ab288be5.java)
					$ServletRequestServicer_188ab288bed.service($ServletRequestServicer_188ab288bed.java)
					org.apache.tapestry.services.impl.SetupRequestEncoding.service(SetupRequestEncoding.java:53)
					$ServletRequestServicerFilter_188ab288be9.service($ServletRequestServicerFilter_188ab288be9.java)
					$ServletRequestServicer_188ab288bed.service($ServletRequestServicer_188ab288bed.java)
					$ServletRequestServicer_188ab288bdf.service($ServletRequestServicer_188ab288bdf.java)
					org.apache.tapestry.ApplicationServlet.doService(ApplicationServlet.java:126)
					org.apache.tapestry.ApplicationServlet.doPost(ApplicationServlet.java:171)
					javax.servlet.http.HttpServlet.service(HttpServlet.java:643)
					javax.servlet.http.HttpServlet.service(HttpServlet.java:723)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:290)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					org.apache.tapestry.RedirectFilter.doFilter(RedirectFilter.java:103)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:235)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					org.tuckey.web.filters.urlrewrite.RuleChain.handleRewrite(RuleChain.java:176)
					org.tuckey.web.filters.urlrewrite.RuleChain.doRules(RuleChain.java:145)
					org.tuckey.web.filters.urlrewrite.UrlRewriter.processRequest(UrlRewriter.java:92)
					org.tuckey.web.filters.urlrewrite.UrlRewriteFilter.doFilter(UrlRewriteFilter.java:381)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:235)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					ime.tinread.web.servlet.filters.HTTPCacheFilter.doFilter(HTTPCacheFilter.java:26)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:235)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:359)
					org.springframework.security.intercept.web.FilterSecurityInterceptor.invoke(FilterSecurityInterceptor.java:109)
					org.springframework.security.intercept.web.FilterSecurityInterceptor.doFilter(FilterSecurityInterceptor.java:83)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.ui.SessionFixationProtectionFilter.doFilterHttp(SessionFixationProtectionFilter.java:67)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.ui.ExceptionTranslationFilter.doFilterHttp(ExceptionTranslationFilter.java:101)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.providers.anonymous.AnonymousProcessingFilter.doFilterHttp(AnonymousProcessingFilter.java:105)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.ui.rememberme.RememberMeProcessingFilter.doFilterHttp(RememberMeProcessingFilter.java:116)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.wrapper.SecurityContextHolderAwareRequestFilter.doFilterHttp(SecurityContextHolderAwareRequestFilter.java:91)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.ui.basicauth.BasicProcessingFilter.doFilterHttp(BasicProcessingFilter.java:173)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.ui.AbstractProcessingFilter.doFilterHttp(AbstractProcessingFilter.java:271)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.ui.logout.LogoutFilter.doFilterHttp(LogoutFilter.java:89)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.context.HttpSessionContextIntegrationFilter.doFilterHttp(HttpSessionContextIntegrationFilter.java:235)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.util.FilterChainProxy.doFilter(FilterChainProxy.java:174)
					org.springframework.web.filter.DelegatingFilterProxy.invokeDelegate(DelegatingFilterProxy.java:236)
					org.springframework.web.filter.DelegatingFilterProxy.doFilter(DelegatingFilterProxy.java:167)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:235)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					ime.tinread.servlets.LogbackFilter.doFilter(LogbackFilter.java:28)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:235)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					transtech.webutil.DisableUrlSessionFilter.doFilter(DisableUrlSessionFilter.java:58)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:235)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					transtech.webutil.IPAccessFilter.doFilter(IPAccessFilter.java:95)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:235)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					org.apache.catalina.core.StandardWrapperValve.invoke(StandardWrapperValve.java:233)
					org.apache.catalina.core.StandardContextValve.invoke(StandardContextValve.java:191)
					org.apache.catalina.core.StandardHostValve.invoke(StandardHostValve.java:127)
					org.apache.catalina.valves.ErrorReportValve.invoke(ErrorReportValve.java:103)
					org.apache.catalina.core.StandardEngineValve.invoke(StandardEngineValve.java:109)
					org.apache.catalina.connector.CoyoteAdapter.service(CoyoteAdapter.java:293)
					org.apache.coyote.http11.Http11Processor.process(Http11Processor.java:861)
					org.apache.coyote.http11.Http11Protocol$Http11ConnectionHandler.process(Http11Protocol.java:620)
					org.apache.tomcat.util.net.JIoEndpoint$Worker.run(JIoEndpoint.java:489)
					java.lang.Thread.run(Thread.java:662)
					
					
			
		
 
	
	
 


      
      
      
      
      
         [+/-]  請顯示訊息 
      
      
      
      
    
    
  

&quot;) or . = concat(&quot;


  
    
  
    
      
        
          An exception has occurred!
        
        
          
            請重新登入系統!
          
        
      
      
      
      
        

    [ +/- ] Exception:
	   Unable to update OGNL expression &quot; , &quot;'&quot; , &quot;&lt;parsed OGNL expression>&quot; , &quot;'&quot; , &quot; of $AdminInventoryRule_143@3c1[AdminInventoryRule] to 1234567890abc: For input string: &quot;1234567890abc&quot;	
	
		
			org.apache.tapestry.BindingException
		
		
		
			binding:
			ExpressionBinding[AdminInventoryRule editRule.inventoryDigits]

		
			location:
			context:/WEB-INF/AdminInventoryRule.html, line 154149      &lt;/tr>
150        &lt;tr>
151          &lt;td class=&quot;table1_left&quot; width = &quot;25%&quot;>&lt;span jwcid=&quot;@Insert&quot; value=&quot;ognl:getMessage(&quot; , &quot;'&quot; , &quot;AdminInventoryRule.41837&quot; , &quot;'&quot; , &quot;)&quot;>Card Digits&lt;/span>:&lt;/td>
152          &lt;td jwcid=&quot;DigitsField@Any&quot;>
153          &lt;span jwcid=&quot;@Insert&quot; value=&quot;ognl:requiredDigitsField&quot; renderTag=&quot;true&quot; style=&quot;color: red;&quot;>&lt;/span>
154          &lt;input jwcid=&quot;@TextField&quot; value=&quot;ognl:editRule.inventoryDigits&quot; validators=&quot;validators:required&quot; onkeyup=&quot;verifyNumber(this)&quot; size=&quot;8&quot;/>&lt;font color=&quot;red&quot;>*&lt;/font>
155          &lt;/td>
156          
157          
158        &lt;/tr>
159        &lt;tr width=&quot;100%&quot; align=&quot;right&quot;>

		

		     

 
	
	

    [ +/- ] Exception:
	   Unable to update OGNL expression &quot; , &quot;'&quot; , &quot;&lt;parsed OGNL expression>&quot; , &quot;'&quot; , &quot; of $AdminInventoryRule_143@3c1[AdminInventoryRule] to 1234567890abc: For input string: &quot;1234567890abc&quot;	
	
		
			org.apache.hivemind.ApplicationRuntimeException
		
		
		
			component:
			$AdminInventoryRule_143@3c1[AdminInventoryRule]

		
			location:
			context:/WEB-INF/AdminInventoryRule.page, line 4, column 601&lt;?xml version=&quot;1.0&quot; encoding=&quot;UTF-8&quot;?>
2&lt;!DOCTYPE page-specification PUBLIC &quot;-//Apache Software Foundation//Tapestry Specification 4.0//EN&quot; &quot;http://jakarta.apache.org/tapestry/dtd/Tapestry_4_0.dtd&quot;>
3
4&lt;page-specification class=&quot;sibimol.web.AdminInventoryRule&quot;>
5    &lt;asset name=&quot;js_utility_js&quot; path=&quot;context:/js/utility.js&quot;/>
6    &lt;asset name=&quot;images_arrow_checked_13_png&quot; path=&quot;context:/images/arrow_checked_13.png&quot;/>
7    &lt;asset name=&quot;images_arrow_unchecked_13_png&quot; path=&quot;context:/images/arrow_unchecked_13.png&quot;/>
8&lt;/page-specification>

		

		     

 
	
	

    [ +/- ] Exception:
	   For input string: &quot;1234567890abc&quot;	
	
		
			java.lang.NumberFormatException
		
		
		

		


		
			Stack Trace:
		

		
			
				
					java.lang.NumberFormatException.forInputString(NumberFormatException.java:48)
					java.lang.Long.parseLong(Long.java:419)
					java.lang.Long.parseLong(Long.java:468)
					ognl.OgnlOps.longValue(OgnlOps.java:220)
					ognl.OgnlOps.convertValue(OgnlOps.java:587)
					ognl.OgnlOps.convertValue(OgnlOps.java:509)
					$ASTChain_188ab28964d.ref1($ASTChain_188ab28964d.java)
					$ASTChain_188ab28964d.set($ASTChain_188ab28964d.java)
					org.apache.tapestry.services.impl.ExpressionEvaluatorImpl.write(ExpressionEvaluatorImpl.java:178)
					$ExpressionEvaluator_188ab288d27.write($ExpressionEvaluator_188ab288d27.java)
					org.apache.tapestry.binding.ExpressionBinding.setObject(ExpressionBinding.java:224)
					$TextField_9.setValue($TextField_9.java)
					org.apache.tapestry.form.TextField.rewindFormComponent(TextField.java:105)
					org.apache.tapestry.form.AbstractFormComponent.renderComponent(AbstractFormComponent.java:90)
					org.apache.tapestry.AbstractComponent.render(AbstractComponent.java:724)
					org.apache.tapestry.services.impl.DojoAjaxResponseBuilder.render(DojoAjaxResponseBuilder.java:635)
					org.apache.tapestry.AbstractComponent.renderBody(AbstractComponent.java:538)
					org.apache.tapestry.components.Any.renderComponent(Any.java:44)
					org.apache.tapestry.AbstractComponent.render(AbstractComponent.java:724)
					org.apache.tapestry.services.impl.DojoAjaxResponseBuilder.render(DojoAjaxResponseBuilder.java:635)
					org.apache.tapestry.AbstractComponent.renderBody(AbstractComponent.java:538)
					org.apache.tapestry.form.FormSupportImpl.rewind(FormSupportImpl.java:624)
					org.apache.tapestry.form.Form.renderComponent(Form.java:196)
					org.apache.tapestry.AbstractComponent.render(AbstractComponent.java:724)
					org.apache.tapestry.services.impl.DojoAjaxResponseBuilder.render(DojoAjaxResponseBuilder.java:635)
					org.apache.tapestry.form.Form.rewind(Form.java:269)
					org.apache.tapestry.engine.RequestCycle.rewindForm(RequestCycle.java:469)
					org.apache.tapestry.form.Form.trigger(Form.java:280)
					org.apache.tapestry.engine.DirectService.triggerComponent(DirectService.java:166)
					org.apache.tapestry.engine.DirectService.service(DirectService.java:142)
					$IEngineService_188ab288c36.service($IEngineService_188ab288c36.java)
					org.apache.tapestry.services.impl.EngineServiceOuterProxy.service(EngineServiceOuterProxy.java:72)
					org.apache.tapestry.engine.AbstractEngine.service(AbstractEngine.java:241)
					org.apache.tapestry.services.impl.InvokeEngineTerminator.service(InvokeEngineTerminator.java:54)
					$WebRequestServicer_188ab288c09.service($WebRequestServicer_188ab288c09.java)
					$WebRequestServicer_188ab288c05.service($WebRequestServicer_188ab288c05.java)
					org.apache.tapestry.services.impl.WebRequestServicerPipelineBridge.service(WebRequestServicerPipelineBridge.java:61)
					$ServletRequestServicer_188ab288beb.service($ServletRequestServicer_188ab288beb.java)
					org.apache.tapestry.request.DecodedRequestInjector.service(DecodedRequestInjector.java:55)
					$ServletRequestServicerFilter_188ab288be7.service($ServletRequestServicerFilter_188ab288be7.java)
					$ServletRequestServicer_188ab288bed.service($ServletRequestServicer_188ab288bed.java)
					org.apache.tapestry.multipart.MultipartDecoderFilter.service(MultipartDecoderFilter.java:52)
					$ServletRequestServicerFilter_188ab288be5.service($ServletRequestServicerFilter_188ab288be5.java)
					$ServletRequestServicer_188ab288bed.service($ServletRequestServicer_188ab288bed.java)
					org.apache.tapestry.services.impl.SetupRequestEncoding.service(SetupRequestEncoding.java:53)
					$ServletRequestServicerFilter_188ab288be9.service($ServletRequestServicerFilter_188ab288be9.java)
					$ServletRequestServicer_188ab288bed.service($ServletRequestServicer_188ab288bed.java)
					$ServletRequestServicer_188ab288bdf.service($ServletRequestServicer_188ab288bdf.java)
					org.apache.tapestry.ApplicationServlet.doService(ApplicationServlet.java:126)
					org.apache.tapestry.ApplicationServlet.doPost(ApplicationServlet.java:171)
					javax.servlet.http.HttpServlet.service(HttpServlet.java:643)
					javax.servlet.http.HttpServlet.service(HttpServlet.java:723)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:290)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					org.apache.tapestry.RedirectFilter.doFilter(RedirectFilter.java:103)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:235)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					org.tuckey.web.filters.urlrewrite.RuleChain.handleRewrite(RuleChain.java:176)
					org.tuckey.web.filters.urlrewrite.RuleChain.doRules(RuleChain.java:145)
					org.tuckey.web.filters.urlrewrite.UrlRewriter.processRequest(UrlRewriter.java:92)
					org.tuckey.web.filters.urlrewrite.UrlRewriteFilter.doFilter(UrlRewriteFilter.java:381)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:235)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					ime.tinread.web.servlet.filters.HTTPCacheFilter.doFilter(HTTPCacheFilter.java:26)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:235)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:359)
					org.springframework.security.intercept.web.FilterSecurityInterceptor.invoke(FilterSecurityInterceptor.java:109)
					org.springframework.security.intercept.web.FilterSecurityInterceptor.doFilter(FilterSecurityInterceptor.java:83)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.ui.SessionFixationProtectionFilter.doFilterHttp(SessionFixationProtectionFilter.java:67)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.ui.ExceptionTranslationFilter.doFilterHttp(ExceptionTranslationFilter.java:101)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.providers.anonymous.AnonymousProcessingFilter.doFilterHttp(AnonymousProcessingFilter.java:105)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.ui.rememberme.RememberMeProcessingFilter.doFilterHttp(RememberMeProcessingFilter.java:116)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.wrapper.SecurityContextHolderAwareRequestFilter.doFilterHttp(SecurityContextHolderAwareRequestFilter.java:91)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.ui.basicauth.BasicProcessingFilter.doFilterHttp(BasicProcessingFilter.java:173)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.ui.AbstractProcessingFilter.doFilterHttp(AbstractProcessingFilter.java:271)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.ui.logout.LogoutFilter.doFilterHttp(LogoutFilter.java:89)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.context.HttpSessionContextIntegrationFilter.doFilterHttp(HttpSessionContextIntegrationFilter.java:235)
					org.springframework.security.ui.SpringSecurityFilter.doFilter(SpringSecurityFilter.java:53)
					org.springframework.security.util.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:371)
					org.springframework.security.util.FilterChainProxy.doFilter(FilterChainProxy.java:174)
					org.springframework.web.filter.DelegatingFilterProxy.invokeDelegate(DelegatingFilterProxy.java:236)
					org.springframework.web.filter.DelegatingFilterProxy.doFilter(DelegatingFilterProxy.java:167)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:235)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					ime.tinread.servlets.LogbackFilter.doFilter(LogbackFilter.java:28)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:235)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					transtech.webutil.DisableUrlSessionFilter.doFilter(DisableUrlSessionFilter.java:58)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:235)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					transtech.webutil.IPAccessFilter.doFilter(IPAccessFilter.java:95)
					org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(ApplicationFilterChain.java:235)
					org.apache.catalina.core.ApplicationFilterChain.doFilter(ApplicationFilterChain.java:206)
					org.apache.catalina.core.StandardWrapperValve.invoke(StandardWrapperValve.java:233)
					org.apache.catalina.core.StandardContextValve.invoke(StandardContextValve.java:191)
					org.apache.catalina.core.StandardHostValve.invoke(StandardHostValve.java:127)
					org.apache.catalina.valves.ErrorReportValve.invoke(ErrorReportValve.java:103)
					org.apache.catalina.core.StandardEngineValve.invoke(StandardEngineValve.java:109)
					org.apache.catalina.connector.CoyoteAdapter.service(CoyoteAdapter.java:293)
					org.apache.coyote.http11.Http11Processor.process(Http11Processor.java:861)
					org.apache.coyote.http11.Http11Protocol$Http11ConnectionHandler.process(Http11Protocol.java:620)
					org.apache.tomcat.util.net.JIoEndpoint$Worker.run(JIoEndpoint.java:489)
					java.lang.Thread.run(Thread.java:662)
					
					
			
		
 
	
	
 


      
      
      
      
      
         [+/-]  請顯示訊息 
      
      
      
      
    
    
  

&quot;))]</value>
      <webElementGuid>afb23005-cd9d-4961-b52f-c14ec2df1fdb</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
