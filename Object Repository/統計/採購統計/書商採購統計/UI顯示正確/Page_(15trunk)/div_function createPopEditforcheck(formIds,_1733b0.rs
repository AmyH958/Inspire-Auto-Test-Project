<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_function createPopEditforcheck(formIds,_1733b0</name>
   <tag></tag>
   <elementGuidId>8f823957-5f2f-448b-85a1-3ed3fae5df29</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//body[@id='Body']/div/div/div[2]</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>div.main_right.l</value>
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
      <webElementGuid>aa2bec31-624f-44b1-936f-5fc36a1c47e0</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>main_right l</value>
      <webElementGuid>b0c4a656-77b5-422c-956c-a384386715c0</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
				統計 > 採購統計 > 書商採購統計		
			


	function createPopEditforcheck(formIds, href){
		hrefparameters = addCheck(formIds);
		if (hrefparameters!=null){
			popupwindow = window.open(&quot;&quot; ,&quot;MeniuCatalogare&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,width=840,height=600&quot;);
			popupwindow.moveTo(screen.width/2-420, screen.height/2-300);
			popupwindow.focus();
			popupwindow.location = href+hrefparameters;
		}
		return false;
	}

	function addCheck(formIds){
		len = document.getElementById(formIds).elements.length;
		total=0;
		href=&quot;&quot;;

		for(var i=0;i&lt;len;i++) {
			var item = document.getElementById(formIds).elements[i];

			if ((item.name.indexOf(&quot;selectat&quot;) == 0)&amp;&amp;(item.checked == true)) {
				id = document.getElementById(formIds).elements[i-1];
				href = href+&quot;&amp;sp=&quot;+id.value;
				total++;
			}
		}

		if (total>1){
			alert(&quot;請選擇一筆記錄！&quot;);
			return null;
		}else if (total==1) return href;
		else return null;
	}

	function changeStatus(element){

	var scheduledData = document.getElementById(&quot;ScheduledData&quot;);

	if (element.checked==true)  scheduledData.style.display ='';
	else scheduledData.style.display ='none';
	}

	function closeDialog() {
        dojo.widget.byId('DialogContent').hide();
    }



  div.auto_complete {
    width: 350px;
    background: #fff;
  }
  div.auto_complete ul {
    border:1px solid #888;
    margin:0;
    padding:0;
    width:100%;
    list-style-type:none;
  }
  div.auto_complete ul li {
    margin:0;
    padding:3px;
  }
  div.auto_complete ul li.selected {
    background-color: #ffb;
  }
  div.auto_complete ul strong.highlight {
    color: #800;
    margin:0;
    padding:0;
  }








































  
  
    
	  書商名稱:
	  
    
    
      採購日期:
	    -  
    
    
      
          
          
              清除
          
      
	
	
       
    
    
      
		每頁筆數:
	    
10
20
50

	  
	   
  	  
	    
		  
		    [1 - 0] 起自 0 查到結果 ( sec)
		  
	    
  	  
    
	
        
			
			  
				
			 	  
			  		
			  		  
  			  		    序號
			  			功能

			  			書商名稱
			  			擬購冊數
			  			實購冊數
			  			缺書數
			  			採購經費
					  
			  		  
			  			
			  				-19
			  			
			  			
			  				
			  			
			  			
			  			   1
			  			
			  			
			     		    2046
			     		
			  			
			     		    2046
			     		
			  			
			     		    3480
			     		
			  			
			     		    1.49568E7
			     		
			  		  
			  			
			  				-18
			  			
			  			
			  				
			  			
			  			
			  			   123測試供應商簡體
			  			
			  			
			     		    4092
			     		
			  			
			     		    3069
			     		
			  			
			     		    6090
			     		
			  			
			     		    2.61744E7
			     		
			  		  
			  			
			  				-17
			  			
			  			
			  				
			  			
			  			
			  			   20220224
			  			
			  			
			     		    1023
			     		
			  			
			     		    0
			     		
			  			
			     		    870
			     		
			  			
			     		    3739200.0
			     		
			  		  
			  			
			  				-16
			  			
			  			
			  				
			  			
			  			
			  			   21世紀文化0327test
			  			
			  			
			     		    0
			     		
			  			
			     		    2046
			     		
			  			
			     		    1740
			     		
			  			
			     		    7478400.0
			     		
			  		  
			  			
			  				-15
			  			
			  			
			  				
			  			
			  			
			  			   333
			  			
			  			
			     		    1023
			     		
			  			
			     		    0
			     		
			  			
			     		    870
			     		
			  			
			     		    3739200.0
			     		
			  		  
			  			
			  				-14
			  			
			  			
			  				
			  			
			  			
			  			   YIFU
			  			
			  			
			     		    0
			     		
			  			
			     		    1023
			     		
			  			
			     		    870
			     		
			  			
			     		    3739200.0
			     		
			  		  
			  			
			  				-13
			  			
			  			
			  				
			  			
			  			
			  			   三民書局1
			  			
			  			
			     		    3069
			     		
			  			
			     		    8184
			     		
			  			
			     		    9570
			     		
			  			
			     		    4.11312E7
			     		
			  		  
			  			
			  				-12
			  			
			  			
			  				
			  			
			  			
			  			   久昌
			  			
			  			
			     		    1023
			     		
			  			
			     		    6138
			     		
			  			
			     		    6090
			     		
			  			
			     		    2.61744E7
			     		
			  		  
			  			
			  				-11
			  			
			  			
			  				
			  			
			  			
			  			   力大
			  			
			  			
			     		    1023
			     		
			  			
			     		    4092
			     		
			  			
			     		    4350
			     		
			  			
			     		    1.8696E7
			     		
			  		  
			  			
			  				-10
			  			
			  			
			  				
			  			
			  			
			  			   力大, 
			  			
			  			
			     		    2046
			     		
			  			
			     		    1023
			     		
			  			
			     		    2610
			     		
			  			
			     		    1.12176E7
			     		
			  		  
			  			
			  				-9
			  			
			  			
			  				
			  			
			  			
			  			   宇勗
			  			
			  			
			     		    0
			     		
			  			
			     		    1023
			     		
			  			
			     		    870
			     		
			  			
			     		    3739200.0
			     		
			  		  
			  			
			  				-8
			  			
			  			
			  				
			  			
			  			
			  			   宏寶
			  			
			  			
			     		    1023
			     		
			  			
			     		    3069
			     		
			  			
			     		    3480
			     		
			  			
			     		    1.49568E7
			     		
			  		  
			  			
			  				-7
			  			
			  			
			  				
			  			
			  			
			  			   尼克
			  			
			  			
			     		    1023
			     		
			  			
			     		    3069
			     		
			  			
			     		    3480
			     		
			  			
			     		    1.49568E7
			     		
			  		  
			  			
			  				-6
			  			
			  			
			  				
			  			
			  			
			  			   崩潰的書
			  			
			  			
			     		    1023
			     		
			  			
			     		    0
			     		
			  			
			     		    870
			     		
			  			
			     		    3739200.0
			     		
			  		  
			  			
			  				-5
			  			
			  			
			  				
			  			
			  			
			  			   巨柏文化事業有限公司
			  			
			  			
			     		    2046
			     		
			  			
			     		    19437
			     		
			  			
			     		    18270
			     		
			  			
			     		    7.85232E7
			     		
			  		  
			  			
			  				-4
			  			
			  			
			  				
			  			
			  			
			  			   心花朵朵1
			  			
			  			
			     		    1023
			     		
			  			
			     		    1023
			     		
			  			
			     		    1740
			     		
			  			
			     		    7478400.0
			     		
			  		  
			  			
			  				-3
			  			
			  			
			  				
			  			
			  			
			  			   我们
			  			
			  			
			     		    1023
			     		
			  			
			     		    1023
			     		
			  			
			     		    1740
			     		
			  			
			     		    7478400.0
			     		
			  		  
			  			
			  				-2
			  			
			  			
			  				
			  			
			  			
			  			   文景
			  			
			  			
			     		    0
			     		
			  			
			     		    5115
			     		
			  			
			     		    4350
			     		
			  			
			     		    1.8696E7
			     		
			  		  
			  			
			  				-1
			  			
			  			
			  				
			  			
			  			
			  			   文書商
			  			
			  			
			     		    0
			     		
			  			
			     		    1023
			     		
			  			
			     		    870
			     		
			  			
			     		    3739200.0
			     		
			  		  
			  			
			  				0
			  			
			  			
			  				
			  			
			  			
			  			   書商20231022
			  			
			  			
			     		    0
			     		
			  			
			     		    1023
			     		
			  			
			     		    870
			     		
			  			
			     		    3739200.0
			     		
			  		  
					
				  
				

				
				  
					
					  

					    
			     			頁數:
		         			
		             		
		              		
			          	
					  
					
				  
		  		
		   	  

        	  
			
		
	
  
  
   			


	報表格式 :
	
SupplierAcquisitionStatReport.jrxml

	
	排程的報表:
		
		
	
		
			
				 設定報表單次執行時間

			
				
				
					  
	
		
			
				
					
						
							
							
								
									
								
								
									
								
								
									
								
								
									
								
								四月
							
							
						
					
				
			
		
		
			
				
					
						
							
								日
								一
								二
								三
								四
								五
								六
							
						
						
								31
								1
								2
								3
								4
								5
								6
							
								7
								8
								9
								10
								11
								12
								13
							
								14
								15
								16
								17
								18
								19
								20
							
								21
								22
								23
								24
								25
								26
								27
							
								28
								29
								30
								1
								2
								3
								4
							
								5
								6
								7
								8
								9
								10
								11
							
					
				
			
		
		
			
				
					
						
							
							
								
									2023年
									2024年
									2025年
								
							
							
						
					
				
			
		
	
	

  
	
		
			
				 
				小時
				分鐘
				 
			
		
		
			
				
					
						
							
								12
								6
							
							
								1
								7
							
							
								2
								8
							
							
								3
								9
							
							
								4
								10
							
							
								5
								11
							
						
					
				
				
					
						
							
								00
								30
							
							
								05
								35
							
							
								10
								40
							
							
								15
								45
							
							
								20
								50
							
							
								25
								55
							
						
					
				
			
			
				 
				
					
						
							
								上午
								下午
							
						
					
				
				
					any
				
				 
			
		
	


				
					

2024
2025
2026
2027
2028
2029
2030
2031
2032
2033
2034
2035
2036
2037
2038
2039
2040
2041
2042
2043
2044
2045
2046
2047
2048
2049
2050
2051
2052
2053
2054

					
任一
1
2
3
4
5
6
7
8
9
10
11
12

					

任一
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31

					

任一
星期一
星期二
星期三
星期四
星期五
星期六
星期天

					
任一
0
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23

					
任一
0
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59

				 
		
	


	報表輸出:
	
CSV_BIG5
PDF
XLS
RTF
HTML
CSV_UTF8




 
   				
					
				  
				  報表
				  報表狀態 
					
				
				 


	var refreshTimeout = null;
	function refresh() {
		clickDirectLink('refresher');
		setstytle();
	};
	function setstytle() {
		var ReportStatus = document.getElementById(&quot;ReportStatus&quot;);
		ReportStatus.style.width = &quot;663px&quot;;
		ReportStatus.style.height = &quot;360px&quot;;
	};



 
  
    報表狀態
  
  
 
 
  
 


  
  
  






 
  
    報表
  
  
 
 
  
 



 
  
    Report Status
  
  
 
 
  
 






     處理中...



			</value>
      <webElementGuid>4c7c1c94-0a6f-4081-836d-4e3a350fa04c</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;Body&quot;)/div[@class=&quot;wrap&quot;]/div[@class=&quot;main l&quot;]/div[@class=&quot;main_right l&quot;]</value>
      <webElementGuid>e82e7453-ac29-4e10-a1ae-c9d8bae8a239</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//body[@id='Body']/div/div/div[2]</value>
      <webElementGuid>e8972948-b4a9-4ff1-be6b-9e931a628af9</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='電子資源狀況報表'])[1]/following::div[1]</value>
      <webElementGuid>ad3f2510-ca6f-4dee-aa25-e3260400203c</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='查詢電子資源狀態'])[1]/following::div[1]</value>
      <webElementGuid>11233522-5cae-44a4-80cf-a0f0eb0f46a5</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div/div[2]</value>
      <webElementGuid>0a816d13-35e8-4bb7-8c35-19b75cdcff15</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;
				統計 > 採購統計 > 書商採購統計		
			


	function createPopEditforcheck(formIds, href){
		hrefparameters = addCheck(formIds);
		if (hrefparameters!=null){
			popupwindow = window.open(&quot;&quot; ,&quot;MeniuCatalogare&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,width=840,height=600&quot;);
			popupwindow.moveTo(screen.width/2-420, screen.height/2-300);
			popupwindow.focus();
			popupwindow.location = href+hrefparameters;
		}
		return false;
	}

	function addCheck(formIds){
		len = document.getElementById(formIds).elements.length;
		total=0;
		href=&quot;&quot;;

		for(var i=0;i&lt;len;i++) {
			var item = document.getElementById(formIds).elements[i];

			if ((item.name.indexOf(&quot;selectat&quot;) == 0)&amp;&amp;(item.checked == true)) {
				id = document.getElementById(formIds).elements[i-1];
				href = href+&quot;&amp;sp=&quot;+id.value;
				total++;
			}
		}

		if (total>1){
			alert(&quot;請選擇一筆記錄！&quot;);
			return null;
		}else if (total==1) return href;
		else return null;
	}

	function changeStatus(element){

	var scheduledData = document.getElementById(&quot;ScheduledData&quot;);

	if (element.checked==true)  scheduledData.style.display =&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	else scheduledData.style.display =&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
	}

	function closeDialog() {
        dojo.widget.byId(&quot; , &quot;'&quot; , &quot;DialogContent&quot; , &quot;'&quot; , &quot;).hide();
    }



  div.auto_complete {
    width: 350px;
    background: #fff;
  }
  div.auto_complete ul {
    border:1px solid #888;
    margin:0;
    padding:0;
    width:100%;
    list-style-type:none;
  }
  div.auto_complete ul li {
    margin:0;
    padding:3px;
  }
  div.auto_complete ul li.selected {
    background-color: #ffb;
  }
  div.auto_complete ul strong.highlight {
    color: #800;
    margin:0;
    padding:0;
  }








































  
  
    
	  書商名稱:
	  
    
    
      採購日期:
	    -  
    
    
      
          
          
              清除
          
      
	
	
       
    
    
      
		每頁筆數:
	    
10
20
50

	  
	   
  	  
	    
		  
		    [1 - 0] 起自 0 查到結果 ( sec)
		  
	    
  	  
    
	
        
			
			  
				
			 	  
			  		
			  		  
  			  		    序號
			  			功能

			  			書商名稱
			  			擬購冊數
			  			實購冊數
			  			缺書數
			  			採購經費
					  
			  		  
			  			
			  				-19
			  			
			  			
			  				
			  			
			  			
			  			   1
			  			
			  			
			     		    2046
			     		
			  			
			     		    2046
			     		
			  			
			     		    3480
			     		
			  			
			     		    1.49568E7
			     		
			  		  
			  			
			  				-18
			  			
			  			
			  				
			  			
			  			
			  			   123測試供應商簡體
			  			
			  			
			     		    4092
			     		
			  			
			     		    3069
			     		
			  			
			     		    6090
			     		
			  			
			     		    2.61744E7
			     		
			  		  
			  			
			  				-17
			  			
			  			
			  				
			  			
			  			
			  			   20220224
			  			
			  			
			     		    1023
			     		
			  			
			     		    0
			     		
			  			
			     		    870
			     		
			  			
			     		    3739200.0
			     		
			  		  
			  			
			  				-16
			  			
			  			
			  				
			  			
			  			
			  			   21世紀文化0327test
			  			
			  			
			     		    0
			     		
			  			
			     		    2046
			     		
			  			
			     		    1740
			     		
			  			
			     		    7478400.0
			     		
			  		  
			  			
			  				-15
			  			
			  			
			  				
			  			
			  			
			  			   333
			  			
			  			
			     		    1023
			     		
			  			
			     		    0
			     		
			  			
			     		    870
			     		
			  			
			     		    3739200.0
			     		
			  		  
			  			
			  				-14
			  			
			  			
			  				
			  			
			  			
			  			   YIFU
			  			
			  			
			     		    0
			     		
			  			
			     		    1023
			     		
			  			
			     		    870
			     		
			  			
			     		    3739200.0
			     		
			  		  
			  			
			  				-13
			  			
			  			
			  				
			  			
			  			
			  			   三民書局1
			  			
			  			
			     		    3069
			     		
			  			
			     		    8184
			     		
			  			
			     		    9570
			     		
			  			
			     		    4.11312E7
			     		
			  		  
			  			
			  				-12
			  			
			  			
			  				
			  			
			  			
			  			   久昌
			  			
			  			
			     		    1023
			     		
			  			
			     		    6138
			     		
			  			
			     		    6090
			     		
			  			
			     		    2.61744E7
			     		
			  		  
			  			
			  				-11
			  			
			  			
			  				
			  			
			  			
			  			   力大
			  			
			  			
			     		    1023
			     		
			  			
			     		    4092
			     		
			  			
			     		    4350
			     		
			  			
			     		    1.8696E7
			     		
			  		  
			  			
			  				-10
			  			
			  			
			  				
			  			
			  			
			  			   力大, 
			  			
			  			
			     		    2046
			     		
			  			
			     		    1023
			     		
			  			
			     		    2610
			     		
			  			
			     		    1.12176E7
			     		
			  		  
			  			
			  				-9
			  			
			  			
			  				
			  			
			  			
			  			   宇勗
			  			
			  			
			     		    0
			     		
			  			
			     		    1023
			     		
			  			
			     		    870
			     		
			  			
			     		    3739200.0
			     		
			  		  
			  			
			  				-8
			  			
			  			
			  				
			  			
			  			
			  			   宏寶
			  			
			  			
			     		    1023
			     		
			  			
			     		    3069
			     		
			  			
			     		    3480
			     		
			  			
			     		    1.49568E7
			     		
			  		  
			  			
			  				-7
			  			
			  			
			  				
			  			
			  			
			  			   尼克
			  			
			  			
			     		    1023
			     		
			  			
			     		    3069
			     		
			  			
			     		    3480
			     		
			  			
			     		    1.49568E7
			     		
			  		  
			  			
			  				-6
			  			
			  			
			  				
			  			
			  			
			  			   崩潰的書
			  			
			  			
			     		    1023
			     		
			  			
			     		    0
			     		
			  			
			     		    870
			     		
			  			
			     		    3739200.0
			     		
			  		  
			  			
			  				-5
			  			
			  			
			  				
			  			
			  			
			  			   巨柏文化事業有限公司
			  			
			  			
			     		    2046
			     		
			  			
			     		    19437
			     		
			  			
			     		    18270
			     		
			  			
			     		    7.85232E7
			     		
			  		  
			  			
			  				-4
			  			
			  			
			  				
			  			
			  			
			  			   心花朵朵1
			  			
			  			
			     		    1023
			     		
			  			
			     		    1023
			     		
			  			
			     		    1740
			     		
			  			
			     		    7478400.0
			     		
			  		  
			  			
			  				-3
			  			
			  			
			  				
			  			
			  			
			  			   我们
			  			
			  			
			     		    1023
			     		
			  			
			     		    1023
			     		
			  			
			     		    1740
			     		
			  			
			     		    7478400.0
			     		
			  		  
			  			
			  				-2
			  			
			  			
			  				
			  			
			  			
			  			   文景
			  			
			  			
			     		    0
			     		
			  			
			     		    5115
			     		
			  			
			     		    4350
			     		
			  			
			     		    1.8696E7
			     		
			  		  
			  			
			  				-1
			  			
			  			
			  				
			  			
			  			
			  			   文書商
			  			
			  			
			     		    0
			     		
			  			
			     		    1023
			     		
			  			
			     		    870
			     		
			  			
			     		    3739200.0
			     		
			  		  
			  			
			  				0
			  			
			  			
			  				
			  			
			  			
			  			   書商20231022
			  			
			  			
			     		    0
			     		
			  			
			     		    1023
			     		
			  			
			     		    870
			     		
			  			
			     		    3739200.0
			     		
			  		  
					
				  
				

				
				  
					
					  

					    
			     			頁數:
		         			
		             		
		              		
			          	
					  
					
				  
		  		
		   	  

        	  
			
		
	
  
  
   			


	報表格式 :
	
SupplierAcquisitionStatReport.jrxml

	
	排程的報表:
		
		
	
		
			
				 設定報表單次執行時間

			
				
				
					  
	
		
			
				
					
						
							
							
								
									
								
								
									
								
								
									
								
								
									
								
								四月
							
							
						
					
				
			
		
		
			
				
					
						
							
								日
								一
								二
								三
								四
								五
								六
							
						
						
								31
								1
								2
								3
								4
								5
								6
							
								7
								8
								9
								10
								11
								12
								13
							
								14
								15
								16
								17
								18
								19
								20
							
								21
								22
								23
								24
								25
								26
								27
							
								28
								29
								30
								1
								2
								3
								4
							
								5
								6
								7
								8
								9
								10
								11
							
					
				
			
		
		
			
				
					
						
							
							
								
									2023年
									2024年
									2025年
								
							
							
						
					
				
			
		
	
	

  
	
		
			
				 
				小時
				分鐘
				 
			
		
		
			
				
					
						
							
								12
								6
							
							
								1
								7
							
							
								2
								8
							
							
								3
								9
							
							
								4
								10
							
							
								5
								11
							
						
					
				
				
					
						
							
								00
								30
							
							
								05
								35
							
							
								10
								40
							
							
								15
								45
							
							
								20
								50
							
							
								25
								55
							
						
					
				
			
			
				 
				
					
						
							
								上午
								下午
							
						
					
				
				
					any
				
				 
			
		
	


				
					

2024
2025
2026
2027
2028
2029
2030
2031
2032
2033
2034
2035
2036
2037
2038
2039
2040
2041
2042
2043
2044
2045
2046
2047
2048
2049
2050
2051
2052
2053
2054

					
任一
1
2
3
4
5
6
7
8
9
10
11
12

					

任一
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31

					

任一
星期一
星期二
星期三
星期四
星期五
星期六
星期天

					
任一
0
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23

					
任一
0
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59

				 
		
	


	報表輸出:
	
CSV_BIG5
PDF
XLS
RTF
HTML
CSV_UTF8




 
   				
					
				  
				  報表
				  報表狀態 
					
				
				 


	var refreshTimeout = null;
	function refresh() {
		clickDirectLink(&quot; , &quot;'&quot; , &quot;refresher&quot; , &quot;'&quot; , &quot;);
		setstytle();
	};
	function setstytle() {
		var ReportStatus = document.getElementById(&quot;ReportStatus&quot;);
		ReportStatus.style.width = &quot;663px&quot;;
		ReportStatus.style.height = &quot;360px&quot;;
	};



 
  
    報表狀態
  
  
 
 
  
 


  
  
  






 
  
    報表
  
  
 
 
  
 



 
  
    Report Status
  
  
 
 
  
 






     處理中...



			&quot;) or . = concat(&quot;
				統計 > 採購統計 > 書商採購統計		
			


	function createPopEditforcheck(formIds, href){
		hrefparameters = addCheck(formIds);
		if (hrefparameters!=null){
			popupwindow = window.open(&quot;&quot; ,&quot;MeniuCatalogare&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,width=840,height=600&quot;);
			popupwindow.moveTo(screen.width/2-420, screen.height/2-300);
			popupwindow.focus();
			popupwindow.location = href+hrefparameters;
		}
		return false;
	}

	function addCheck(formIds){
		len = document.getElementById(formIds).elements.length;
		total=0;
		href=&quot;&quot;;

		for(var i=0;i&lt;len;i++) {
			var item = document.getElementById(formIds).elements[i];

			if ((item.name.indexOf(&quot;selectat&quot;) == 0)&amp;&amp;(item.checked == true)) {
				id = document.getElementById(formIds).elements[i-1];
				href = href+&quot;&amp;sp=&quot;+id.value;
				total++;
			}
		}

		if (total>1){
			alert(&quot;請選擇一筆記錄！&quot;);
			return null;
		}else if (total==1) return href;
		else return null;
	}

	function changeStatus(element){

	var scheduledData = document.getElementById(&quot;ScheduledData&quot;);

	if (element.checked==true)  scheduledData.style.display =&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	else scheduledData.style.display =&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
	}

	function closeDialog() {
        dojo.widget.byId(&quot; , &quot;'&quot; , &quot;DialogContent&quot; , &quot;'&quot; , &quot;).hide();
    }



  div.auto_complete {
    width: 350px;
    background: #fff;
  }
  div.auto_complete ul {
    border:1px solid #888;
    margin:0;
    padding:0;
    width:100%;
    list-style-type:none;
  }
  div.auto_complete ul li {
    margin:0;
    padding:3px;
  }
  div.auto_complete ul li.selected {
    background-color: #ffb;
  }
  div.auto_complete ul strong.highlight {
    color: #800;
    margin:0;
    padding:0;
  }








































  
  
    
	  書商名稱:
	  
    
    
      採購日期:
	    -  
    
    
      
          
          
              清除
          
      
	
	
       
    
    
      
		每頁筆數:
	    
10
20
50

	  
	   
  	  
	    
		  
		    [1 - 0] 起自 0 查到結果 ( sec)
		  
	    
  	  
    
	
        
			
			  
				
			 	  
			  		
			  		  
  			  		    序號
			  			功能

			  			書商名稱
			  			擬購冊數
			  			實購冊數
			  			缺書數
			  			採購經費
					  
			  		  
			  			
			  				-19
			  			
			  			
			  				
			  			
			  			
			  			   1
			  			
			  			
			     		    2046
			     		
			  			
			     		    2046
			     		
			  			
			     		    3480
			     		
			  			
			     		    1.49568E7
			     		
			  		  
			  			
			  				-18
			  			
			  			
			  				
			  			
			  			
			  			   123測試供應商簡體
			  			
			  			
			     		    4092
			     		
			  			
			     		    3069
			     		
			  			
			     		    6090
			     		
			  			
			     		    2.61744E7
			     		
			  		  
			  			
			  				-17
			  			
			  			
			  				
			  			
			  			
			  			   20220224
			  			
			  			
			     		    1023
			     		
			  			
			     		    0
			     		
			  			
			     		    870
			     		
			  			
			     		    3739200.0
			     		
			  		  
			  			
			  				-16
			  			
			  			
			  				
			  			
			  			
			  			   21世紀文化0327test
			  			
			  			
			     		    0
			     		
			  			
			     		    2046
			     		
			  			
			     		    1740
			     		
			  			
			     		    7478400.0
			     		
			  		  
			  			
			  				-15
			  			
			  			
			  				
			  			
			  			
			  			   333
			  			
			  			
			     		    1023
			     		
			  			
			     		    0
			     		
			  			
			     		    870
			     		
			  			
			     		    3739200.0
			     		
			  		  
			  			
			  				-14
			  			
			  			
			  				
			  			
			  			
			  			   YIFU
			  			
			  			
			     		    0
			     		
			  			
			     		    1023
			     		
			  			
			     		    870
			     		
			  			
			     		    3739200.0
			     		
			  		  
			  			
			  				-13
			  			
			  			
			  				
			  			
			  			
			  			   三民書局1
			  			
			  			
			     		    3069
			     		
			  			
			     		    8184
			     		
			  			
			     		    9570
			     		
			  			
			     		    4.11312E7
			     		
			  		  
			  			
			  				-12
			  			
			  			
			  				
			  			
			  			
			  			   久昌
			  			
			  			
			     		    1023
			     		
			  			
			     		    6138
			     		
			  			
			     		    6090
			     		
			  			
			     		    2.61744E7
			     		
			  		  
			  			
			  				-11
			  			
			  			
			  				
			  			
			  			
			  			   力大
			  			
			  			
			     		    1023
			     		
			  			
			     		    4092
			     		
			  			
			     		    4350
			     		
			  			
			     		    1.8696E7
			     		
			  		  
			  			
			  				-10
			  			
			  			
			  				
			  			
			  			
			  			   力大, 
			  			
			  			
			     		    2046
			     		
			  			
			     		    1023
			     		
			  			
			     		    2610
			     		
			  			
			     		    1.12176E7
			     		
			  		  
			  			
			  				-9
			  			
			  			
			  				
			  			
			  			
			  			   宇勗
			  			
			  			
			     		    0
			     		
			  			
			     		    1023
			     		
			  			
			     		    870
			     		
			  			
			     		    3739200.0
			     		
			  		  
			  			
			  				-8
			  			
			  			
			  				
			  			
			  			
			  			   宏寶
			  			
			  			
			     		    1023
			     		
			  			
			     		    3069
			     		
			  			
			     		    3480
			     		
			  			
			     		    1.49568E7
			     		
			  		  
			  			
			  				-7
			  			
			  			
			  				
			  			
			  			
			  			   尼克
			  			
			  			
			     		    1023
			     		
			  			
			     		    3069
			     		
			  			
			     		    3480
			     		
			  			
			     		    1.49568E7
			     		
			  		  
			  			
			  				-6
			  			
			  			
			  				
			  			
			  			
			  			   崩潰的書
			  			
			  			
			     		    1023
			     		
			  			
			     		    0
			     		
			  			
			     		    870
			     		
			  			
			     		    3739200.0
			     		
			  		  
			  			
			  				-5
			  			
			  			
			  				
			  			
			  			
			  			   巨柏文化事業有限公司
			  			
			  			
			     		    2046
			     		
			  			
			     		    19437
			     		
			  			
			     		    18270
			     		
			  			
			     		    7.85232E7
			     		
			  		  
			  			
			  				-4
			  			
			  			
			  				
			  			
			  			
			  			   心花朵朵1
			  			
			  			
			     		    1023
			     		
			  			
			     		    1023
			     		
			  			
			     		    1740
			     		
			  			
			     		    7478400.0
			     		
			  		  
			  			
			  				-3
			  			
			  			
			  				
			  			
			  			
			  			   我们
			  			
			  			
			     		    1023
			     		
			  			
			     		    1023
			     		
			  			
			     		    1740
			     		
			  			
			     		    7478400.0
			     		
			  		  
			  			
			  				-2
			  			
			  			
			  				
			  			
			  			
			  			   文景
			  			
			  			
			     		    0
			     		
			  			
			     		    5115
			     		
			  			
			     		    4350
			     		
			  			
			     		    1.8696E7
			     		
			  		  
			  			
			  				-1
			  			
			  			
			  				
			  			
			  			
			  			   文書商
			  			
			  			
			     		    0
			     		
			  			
			     		    1023
			     		
			  			
			     		    870
			     		
			  			
			     		    3739200.0
			     		
			  		  
			  			
			  				0
			  			
			  			
			  				
			  			
			  			
			  			   書商20231022
			  			
			  			
			     		    0
			     		
			  			
			     		    1023
			     		
			  			
			     		    870
			     		
			  			
			     		    3739200.0
			     		
			  		  
					
				  
				

				
				  
					
					  

					    
			     			頁數:
		         			
		             		
		              		
			          	
					  
					
				  
		  		
		   	  

        	  
			
		
	
  
  
   			


	報表格式 :
	
SupplierAcquisitionStatReport.jrxml

	
	排程的報表:
		
		
	
		
			
				 設定報表單次執行時間

			
				
				
					  
	
		
			
				
					
						
							
							
								
									
								
								
									
								
								
									
								
								
									
								
								四月
							
							
						
					
				
			
		
		
			
				
					
						
							
								日
								一
								二
								三
								四
								五
								六
							
						
						
								31
								1
								2
								3
								4
								5
								6
							
								7
								8
								9
								10
								11
								12
								13
							
								14
								15
								16
								17
								18
								19
								20
							
								21
								22
								23
								24
								25
								26
								27
							
								28
								29
								30
								1
								2
								3
								4
							
								5
								6
								7
								8
								9
								10
								11
							
					
				
			
		
		
			
				
					
						
							
							
								
									2023年
									2024年
									2025年
								
							
							
						
					
				
			
		
	
	

  
	
		
			
				 
				小時
				分鐘
				 
			
		
		
			
				
					
						
							
								12
								6
							
							
								1
								7
							
							
								2
								8
							
							
								3
								9
							
							
								4
								10
							
							
								5
								11
							
						
					
				
				
					
						
							
								00
								30
							
							
								05
								35
							
							
								10
								40
							
							
								15
								45
							
							
								20
								50
							
							
								25
								55
							
						
					
				
			
			
				 
				
					
						
							
								上午
								下午
							
						
					
				
				
					any
				
				 
			
		
	


				
					

2024
2025
2026
2027
2028
2029
2030
2031
2032
2033
2034
2035
2036
2037
2038
2039
2040
2041
2042
2043
2044
2045
2046
2047
2048
2049
2050
2051
2052
2053
2054

					
任一
1
2
3
4
5
6
7
8
9
10
11
12

					

任一
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31

					

任一
星期一
星期二
星期三
星期四
星期五
星期六
星期天

					
任一
0
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23

					
任一
0
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59

				 
		
	


	報表輸出:
	
CSV_BIG5
PDF
XLS
RTF
HTML
CSV_UTF8




 
   				
					
				  
				  報表
				  報表狀態 
					
				
				 


	var refreshTimeout = null;
	function refresh() {
		clickDirectLink(&quot; , &quot;'&quot; , &quot;refresher&quot; , &quot;'&quot; , &quot;);
		setstytle();
	};
	function setstytle() {
		var ReportStatus = document.getElementById(&quot;ReportStatus&quot;);
		ReportStatus.style.width = &quot;663px&quot;;
		ReportStatus.style.height = &quot;360px&quot;;
	};



 
  
    報表狀態
  
  
 
 
  
 


  
  
  






 
  
    報表
  
  
 
 
  
 



 
  
    Report Status
  
  
 
 
  
 






     處理中...



			&quot;))]</value>
      <webElementGuid>b1e549f1-d214-435d-bc44-839b1e8e474d</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
