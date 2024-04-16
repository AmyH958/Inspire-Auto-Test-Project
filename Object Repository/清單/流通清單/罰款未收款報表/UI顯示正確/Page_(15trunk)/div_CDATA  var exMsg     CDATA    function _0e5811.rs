<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_CDATA  var exMsg     CDATA    function _0e5811</name>
   <tag></tag>
   <elementGuidId>0fd3f70f-8e2b-4d5a-a190-cd411e37e12f</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='nrResults']</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#nrResults</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>12355fe0-f1ca-4468-9dcd-395d49b4e7ad</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>nrResults</value>
      <webElementGuid>f43af1d0-f8f3-4bfb-8428-a03efa840ee9</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>border w l</value>
      <webElementGuid>99224ffb-53d5-4cb1-ad3b-98a11a9006e3</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
 		
 		
 		
					
						
  

//&lt;![CDATA[

  	var exMsg='頁碼錯誤，請重新輸入'
  
//]]&gt;


   


//&lt;![CDATA[

    function onSelectChange() {         
      if (document.CautareRapida.languages.value == 'ro')
          Tapestry.submit_form('CautareRapida', 'langFirst');
      if (document.CautareRapida.languages.value == 'ru')
          Tapestry.submit_form('CautareRapida', 'langSecond');
      if (document.CautareRapida.languages.value == 'en')
          Tapestry.submit_form('CautareRapida', 'langThird');
      if (document.CautareRapida.languages.value == 'ch')
          Tapestry.submit_form('CautareRapida', 'langFourth');
    }
j(document).ready(function(){
	
    	j(&quot;#cp&quot;).keydown(function (e){
    	    if(e.keyCode == 13){
    	    	gotofunction();
    	    	return false;
    	    }
    	});
    	
    	j(&quot;.pages_btn&quot;).click(function(){
    		gotofunction();
    	});
    	
   		j(&quot;#cp&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp_0&quot;).val(j(&quot;#cp&quot;).val());
   		});
   		
   		j(&quot;#cp_0&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp&quot;).val(j(&quot;#cp_0&quot;).val());
   		});
    	
    });
    
    function gotofunction() {
    	var h=j(&quot;#go2&quot;).attr(&quot;href&quot;);
  		var head=h.substring(0,h.indexOf(&quot;?&quot;));
  		var foot=h.substring(h.indexOf(&quot;&amp;&quot;)+1,h.length);
  		var body=&quot;?sp=&quot;+j(&quot;#cp&quot;).val()+&quot;&amp;&quot;;
  		var hr=head+body+foot;
  		if(isNaN(j(&quot;#cp&quot;).val())){
  			alert(exMsg);
  			document.getElementById(&quot;cp&quot;).value = '1';
  		}else{
  			j(&quot;#go2&quot;).attr(&quot;href&quot;,hr)
      		j(&quot;#go2&quot;).click();
  		}
    }
  
//]]&gt;


						
							
							
								 
								    			
								  
							
							
														
								
								
									
									  
									     1  		       
									   
									               
									
									  
									  
									   
									     2
									   	
									               
									
									  
									  
									   
									     3
									   	
									               
									
									  
									  
									   
									     4
									   	
									               
									
									  
									  
									   
									     5
									   	
									               
									  
								           
								
								...
								
									 
									   16               
									  
								             
								
							
							
							 
									                 
									 
								
							
								
								
								跳至頁碼:								
								 
								
								
							 GO
													
						
						
					
			
					
						
							
								序號
								讀者證號
								讀者姓名
								罰款產生日期
								還書日期
								書目資訊
								條碼號
								罰款原因
								逾期罰款點數
								遺失罰款
								讀者狀態
								單位系所
							
							
								1
								100001506
								張睿恩
								2018/04/08 09:54:54
								2018/04/08 09:54:54
								how we die :
								012010046567
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								2
								100022014
								呂冠賢
								2015/08/18 17:04:00
								2015/08/18 17:04:00
								robbins病理學 :
								011010092938
								逾期
								442
								0.0
								有效讀者
								中醫學系甲組
							
								3
								100022014
								呂冠賢
								2015/08/18 17:05:00
								2015/08/18 17:05:00
								病理學 =
								011010088742
								逾期
								442
								0.0
								有效讀者
								中醫學系甲組
							
								4
								100022023
								葉宗閔
								2018/03/28 19:23:45
								2018/03/28 19:23:45
								精選new toeic全真模擬試題 /
								011010143849
								逾期
								2
								0.0
								有效讀者
								中醫學系甲組
							
								5
								100022023
								葉宗閔
								2018/03/28 19:23:51
								2018/03/28 19:23:49
								精選new toeic全真模擬試題 /
								011010143848
								逾期
								2
								0.0
								有效讀者
								中醫學系甲組
							
								6
								100022023
								葉宗閔
								2018/03/28 19:24:04
								2018/03/28 19:24:02
								new toeic關鍵金色字彙1200 /
								011010140046
								逾期
								2
								0.0
								有效讀者
								中醫學系甲組
							
								7
								101000411
								陳致成
								2016/02/15 08:18:00
								2016/02/15 08:18:00
								奇光下的祕密 /
								011010136304
								逾期
								1194
								0.0
								有效讀者
								牙醫學系
							
								8
								101000411
								陳致成
								2016/02/15 08:18:00
								2016/02/15 08:18:00
								樹屋 /
								011010134723
								逾期
								1194
								0.0
								有效讀者
								牙醫學系
							
								9
								101000411
								陳致成
								2016/02/15 08:18:00
								2016/02/15 08:18:00
								男孩a /
								011010116222
								逾期
								1194
								0.0
								有效讀者
								牙醫學系
							
								10
								101000411
								陳致成
								2016/02/15 08:18:00
								2016/02/15 08:18:00
								穿越時空.地下鐵 /
								011010096564
								逾期
								1194
								0.0
								有效讀者
								牙醫學系
							
								11
								101000411
								陳致成
								2016/02/15 08:19:00
								2016/02/15 08:19:00
								女朋友 男朋友 =
								011010140726
								逾期
								1194
								0.0
								有效讀者
								牙醫學系
							
								12
								101001450
								蕭宇泰
								2018/05/31 08:19:27
								2018/05/30 08:15:00
								混血營英雄 /
								011010161120
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								13
								101001450
								蕭宇泰
								2018/05/31 08:19:29
								2018/05/30 08:15:00
								嚴酷的學校 /
								011010102020
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								14
								101001450
								蕭宇泰
								2018/05/31 08:19:33
								2018/05/30 08:15:00
								邪惡的村子 /
								011010103540
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								15
								101001450
								蕭宇泰
								2018/05/31 08:19:35
								2018/05/30 08:15:00
								恐怖的醫院 /
								011010071411
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								16
								101001450
								蕭宇泰
								2018/05/31 08:19:50
								2018/05/30 08:15:00
								破爛的電梯 /
								011010102088
								逾期
								6
								0.0
								有效讀者
								醫學系其他
							
								17
								101001450
								蕭宇泰
								2018/05/31 08:19:54
								2018/05/30 08:15:00
								糟糕的工廠 /
								011010102019
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								18
								101001450
								蕭宇泰
								2018/05/31 08:19:57
								2018/05/30 08:15:00
								悲慘的開始 /
								011010071348
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								19
								101001450
								蕭宇泰
								2018/05/31 08:19:59
								2018/05/30 08:15:00
								可怕的爬蟲屋 /
								011010104784
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								20
								101001450
								蕭宇泰
								2018/05/31 08:20:01
								2018/05/30 08:15:00
								鬼魅的大窗子 /
								011010101558
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
						
					
				
				
				
	</value>
      <webElementGuid>2c1321c8-8b24-48d6-b4f9-f9f9b960880c</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;searchForm&quot;)/div[@id=&quot;nrResults&quot;]</value>
      <webElementGuid>9c86ca17-a6e3-485a-95ab-e497adfcb391</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='nrResults']</value>
      <webElementGuid>f7888346-f8fd-4fdc-b03b-555bdd415f96</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//form[@id='searchForm']/div[3]</value>
      <webElementGuid>149d331b-760f-4107-b287-c7a00577cfa8</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='遺失罰款'])[2]/following::div[2]</value>
      <webElementGuid>5092cdf9-6cc2-4788-b779-e2f5394597ef</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='逾期罰款點數'])[1]/following::div[2]</value>
      <webElementGuid>65a7e5ae-6ea5-4b1f-8f21-560193b01482</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//form/div[3]</value>
      <webElementGuid>24b916c1-2bfa-4049-acca-e00cf2841100</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[@id = 'nrResults' and (text() = concat(&quot;
 		
 		
 		
					
						
  

//&lt;![CDATA[

  	var exMsg=&quot; , &quot;'&quot; , &quot;頁碼錯誤，請重新輸入&quot; , &quot;'&quot; , &quot;
  
//]]&gt;


   


//&lt;![CDATA[

    function onSelectChange() {         
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ro&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langFirst&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ru&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langSecond&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langThird&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ch&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langFourth&quot; , &quot;'&quot; , &quot;);
    }
j(document).ready(function(){
	
    	j(&quot;#cp&quot;).keydown(function (e){
    	    if(e.keyCode == 13){
    	    	gotofunction();
    	    	return false;
    	    }
    	});
    	
    	j(&quot;.pages_btn&quot;).click(function(){
    		gotofunction();
    	});
    	
   		j(&quot;#cp&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp_0&quot;).val(j(&quot;#cp&quot;).val());
   		});
   		
   		j(&quot;#cp_0&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp&quot;).val(j(&quot;#cp_0&quot;).val());
   		});
    	
    });
    
    function gotofunction() {
    	var h=j(&quot;#go2&quot;).attr(&quot;href&quot;);
  		var head=h.substring(0,h.indexOf(&quot;?&quot;));
  		var foot=h.substring(h.indexOf(&quot;&amp;&quot;)+1,h.length);
  		var body=&quot;?sp=&quot;+j(&quot;#cp&quot;).val()+&quot;&amp;&quot;;
  		var hr=head+body+foot;
  		if(isNaN(j(&quot;#cp&quot;).val())){
  			alert(exMsg);
  			document.getElementById(&quot;cp&quot;).value = &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
  		}else{
  			j(&quot;#go2&quot;).attr(&quot;href&quot;,hr)
      		j(&quot;#go2&quot;).click();
  		}
    }
  
//]]&gt;


						
							
							
								 
								    			
								  
							
							
														
								
								
									
									  
									     1  		       
									   
									               
									
									  
									  
									   
									     2
									   	
									               
									
									  
									  
									   
									     3
									   	
									               
									
									  
									  
									   
									     4
									   	
									               
									
									  
									  
									   
									     5
									   	
									               
									  
								           
								
								...
								
									 
									   16               
									  
								             
								
							
							
							 
									                 
									 
								
							
								
								
								跳至頁碼:								
								 
								
								
							 GO
													
						
						
					
			
					
						
							
								序號
								讀者證號
								讀者姓名
								罰款產生日期
								還書日期
								書目資訊
								條碼號
								罰款原因
								逾期罰款點數
								遺失罰款
								讀者狀態
								單位系所
							
							
								1
								100001506
								張睿恩
								2018/04/08 09:54:54
								2018/04/08 09:54:54
								how we die :
								012010046567
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								2
								100022014
								呂冠賢
								2015/08/18 17:04:00
								2015/08/18 17:04:00
								robbins病理學 :
								011010092938
								逾期
								442
								0.0
								有效讀者
								中醫學系甲組
							
								3
								100022014
								呂冠賢
								2015/08/18 17:05:00
								2015/08/18 17:05:00
								病理學 =
								011010088742
								逾期
								442
								0.0
								有效讀者
								中醫學系甲組
							
								4
								100022023
								葉宗閔
								2018/03/28 19:23:45
								2018/03/28 19:23:45
								精選new toeic全真模擬試題 /
								011010143849
								逾期
								2
								0.0
								有效讀者
								中醫學系甲組
							
								5
								100022023
								葉宗閔
								2018/03/28 19:23:51
								2018/03/28 19:23:49
								精選new toeic全真模擬試題 /
								011010143848
								逾期
								2
								0.0
								有效讀者
								中醫學系甲組
							
								6
								100022023
								葉宗閔
								2018/03/28 19:24:04
								2018/03/28 19:24:02
								new toeic關鍵金色字彙1200 /
								011010140046
								逾期
								2
								0.0
								有效讀者
								中醫學系甲組
							
								7
								101000411
								陳致成
								2016/02/15 08:18:00
								2016/02/15 08:18:00
								奇光下的祕密 /
								011010136304
								逾期
								1194
								0.0
								有效讀者
								牙醫學系
							
								8
								101000411
								陳致成
								2016/02/15 08:18:00
								2016/02/15 08:18:00
								樹屋 /
								011010134723
								逾期
								1194
								0.0
								有效讀者
								牙醫學系
							
								9
								101000411
								陳致成
								2016/02/15 08:18:00
								2016/02/15 08:18:00
								男孩a /
								011010116222
								逾期
								1194
								0.0
								有效讀者
								牙醫學系
							
								10
								101000411
								陳致成
								2016/02/15 08:18:00
								2016/02/15 08:18:00
								穿越時空.地下鐵 /
								011010096564
								逾期
								1194
								0.0
								有效讀者
								牙醫學系
							
								11
								101000411
								陳致成
								2016/02/15 08:19:00
								2016/02/15 08:19:00
								女朋友 男朋友 =
								011010140726
								逾期
								1194
								0.0
								有效讀者
								牙醫學系
							
								12
								101001450
								蕭宇泰
								2018/05/31 08:19:27
								2018/05/30 08:15:00
								混血營英雄 /
								011010161120
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								13
								101001450
								蕭宇泰
								2018/05/31 08:19:29
								2018/05/30 08:15:00
								嚴酷的學校 /
								011010102020
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								14
								101001450
								蕭宇泰
								2018/05/31 08:19:33
								2018/05/30 08:15:00
								邪惡的村子 /
								011010103540
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								15
								101001450
								蕭宇泰
								2018/05/31 08:19:35
								2018/05/30 08:15:00
								恐怖的醫院 /
								011010071411
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								16
								101001450
								蕭宇泰
								2018/05/31 08:19:50
								2018/05/30 08:15:00
								破爛的電梯 /
								011010102088
								逾期
								6
								0.0
								有效讀者
								醫學系其他
							
								17
								101001450
								蕭宇泰
								2018/05/31 08:19:54
								2018/05/30 08:15:00
								糟糕的工廠 /
								011010102019
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								18
								101001450
								蕭宇泰
								2018/05/31 08:19:57
								2018/05/30 08:15:00
								悲慘的開始 /
								011010071348
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								19
								101001450
								蕭宇泰
								2018/05/31 08:19:59
								2018/05/30 08:15:00
								可怕的爬蟲屋 /
								011010104784
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								20
								101001450
								蕭宇泰
								2018/05/31 08:20:01
								2018/05/30 08:15:00
								鬼魅的大窗子 /
								011010101558
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
						
					
				
				
				
	&quot;) or . = concat(&quot;
 		
 		
 		
					
						
  

//&lt;![CDATA[

  	var exMsg=&quot; , &quot;'&quot; , &quot;頁碼錯誤，請重新輸入&quot; , &quot;'&quot; , &quot;
  
//]]&gt;


   


//&lt;![CDATA[

    function onSelectChange() {         
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ro&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langFirst&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ru&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langSecond&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langThird&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ch&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langFourth&quot; , &quot;'&quot; , &quot;);
    }
j(document).ready(function(){
	
    	j(&quot;#cp&quot;).keydown(function (e){
    	    if(e.keyCode == 13){
    	    	gotofunction();
    	    	return false;
    	    }
    	});
    	
    	j(&quot;.pages_btn&quot;).click(function(){
    		gotofunction();
    	});
    	
   		j(&quot;#cp&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp_0&quot;).val(j(&quot;#cp&quot;).val());
   		});
   		
   		j(&quot;#cp_0&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp&quot;).val(j(&quot;#cp_0&quot;).val());
   		});
    	
    });
    
    function gotofunction() {
    	var h=j(&quot;#go2&quot;).attr(&quot;href&quot;);
  		var head=h.substring(0,h.indexOf(&quot;?&quot;));
  		var foot=h.substring(h.indexOf(&quot;&amp;&quot;)+1,h.length);
  		var body=&quot;?sp=&quot;+j(&quot;#cp&quot;).val()+&quot;&amp;&quot;;
  		var hr=head+body+foot;
  		if(isNaN(j(&quot;#cp&quot;).val())){
  			alert(exMsg);
  			document.getElementById(&quot;cp&quot;).value = &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
  		}else{
  			j(&quot;#go2&quot;).attr(&quot;href&quot;,hr)
      		j(&quot;#go2&quot;).click();
  		}
    }
  
//]]&gt;


						
							
							
								 
								    			
								  
							
							
														
								
								
									
									  
									     1  		       
									   
									               
									
									  
									  
									   
									     2
									   	
									               
									
									  
									  
									   
									     3
									   	
									               
									
									  
									  
									   
									     4
									   	
									               
									
									  
									  
									   
									     5
									   	
									               
									  
								           
								
								...
								
									 
									   16               
									  
								             
								
							
							
							 
									                 
									 
								
							
								
								
								跳至頁碼:								
								 
								
								
							 GO
													
						
						
					
			
					
						
							
								序號
								讀者證號
								讀者姓名
								罰款產生日期
								還書日期
								書目資訊
								條碼號
								罰款原因
								逾期罰款點數
								遺失罰款
								讀者狀態
								單位系所
							
							
								1
								100001506
								張睿恩
								2018/04/08 09:54:54
								2018/04/08 09:54:54
								how we die :
								012010046567
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								2
								100022014
								呂冠賢
								2015/08/18 17:04:00
								2015/08/18 17:04:00
								robbins病理學 :
								011010092938
								逾期
								442
								0.0
								有效讀者
								中醫學系甲組
							
								3
								100022014
								呂冠賢
								2015/08/18 17:05:00
								2015/08/18 17:05:00
								病理學 =
								011010088742
								逾期
								442
								0.0
								有效讀者
								中醫學系甲組
							
								4
								100022023
								葉宗閔
								2018/03/28 19:23:45
								2018/03/28 19:23:45
								精選new toeic全真模擬試題 /
								011010143849
								逾期
								2
								0.0
								有效讀者
								中醫學系甲組
							
								5
								100022023
								葉宗閔
								2018/03/28 19:23:51
								2018/03/28 19:23:49
								精選new toeic全真模擬試題 /
								011010143848
								逾期
								2
								0.0
								有效讀者
								中醫學系甲組
							
								6
								100022023
								葉宗閔
								2018/03/28 19:24:04
								2018/03/28 19:24:02
								new toeic關鍵金色字彙1200 /
								011010140046
								逾期
								2
								0.0
								有效讀者
								中醫學系甲組
							
								7
								101000411
								陳致成
								2016/02/15 08:18:00
								2016/02/15 08:18:00
								奇光下的祕密 /
								011010136304
								逾期
								1194
								0.0
								有效讀者
								牙醫學系
							
								8
								101000411
								陳致成
								2016/02/15 08:18:00
								2016/02/15 08:18:00
								樹屋 /
								011010134723
								逾期
								1194
								0.0
								有效讀者
								牙醫學系
							
								9
								101000411
								陳致成
								2016/02/15 08:18:00
								2016/02/15 08:18:00
								男孩a /
								011010116222
								逾期
								1194
								0.0
								有效讀者
								牙醫學系
							
								10
								101000411
								陳致成
								2016/02/15 08:18:00
								2016/02/15 08:18:00
								穿越時空.地下鐵 /
								011010096564
								逾期
								1194
								0.0
								有效讀者
								牙醫學系
							
								11
								101000411
								陳致成
								2016/02/15 08:19:00
								2016/02/15 08:19:00
								女朋友 男朋友 =
								011010140726
								逾期
								1194
								0.0
								有效讀者
								牙醫學系
							
								12
								101001450
								蕭宇泰
								2018/05/31 08:19:27
								2018/05/30 08:15:00
								混血營英雄 /
								011010161120
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								13
								101001450
								蕭宇泰
								2018/05/31 08:19:29
								2018/05/30 08:15:00
								嚴酷的學校 /
								011010102020
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								14
								101001450
								蕭宇泰
								2018/05/31 08:19:33
								2018/05/30 08:15:00
								邪惡的村子 /
								011010103540
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								15
								101001450
								蕭宇泰
								2018/05/31 08:19:35
								2018/05/30 08:15:00
								恐怖的醫院 /
								011010071411
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								16
								101001450
								蕭宇泰
								2018/05/31 08:19:50
								2018/05/30 08:15:00
								破爛的電梯 /
								011010102088
								逾期
								6
								0.0
								有效讀者
								醫學系其他
							
								17
								101001450
								蕭宇泰
								2018/05/31 08:19:54
								2018/05/30 08:15:00
								糟糕的工廠 /
								011010102019
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								18
								101001450
								蕭宇泰
								2018/05/31 08:19:57
								2018/05/30 08:15:00
								悲慘的開始 /
								011010071348
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								19
								101001450
								蕭宇泰
								2018/05/31 08:19:59
								2018/05/30 08:15:00
								可怕的爬蟲屋 /
								011010104784
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
								20
								101001450
								蕭宇泰
								2018/05/31 08:20:01
								2018/05/30 08:15:00
								鬼魅的大窗子 /
								011010101558
								逾期
								14
								0.0
								有效讀者
								醫學系其他
							
						
					
				
				
				
	&quot;))]</value>
      <webElementGuid>6c3de927-7a8e-4307-a1f2-6ce208c7d0a0</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
