import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://10.11.20.15/inspireapp/logon')

WebUI.maximizeWindow()

WebUI.setText(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/input_(15trunk)_member_pwd'), 
    'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a_'), '流通')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a__1'), '盤點')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a__1_2'), '盤點維護')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a__1_2_3'), '建立範本')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a__1_2_3'))

WebUI.setText(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/input__field1'), '0000000001')

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/select_'), '0', true)

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/input__field1'))

WebUI.setText(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/input__field2'), '0000000018')

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/select_()()()()'), '<=', 
    true)

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/input__formSubmitSearch'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a__1_2_3_4'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/div_Inentory Definition'), 
    'Inentory Definition')

WebUI.verifyElementText(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/td_'), '盤點名稱')

WebUI.setText(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/input__TextField_0'), '測試盤點_20240313')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/div_'))

WebUI.setText(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/textarea__TextArea'), '測試盤點_202403130145')

WebUI.verifyElementText(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a__1_2_3_4_5'), '修改/存檔')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a__1_2_3_4_5'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a__1_2_3_4_5_6'), '回到館藏盤點維護')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a__1_2_3_4_5_6'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a_3'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a__20240313'), '測試盤點_20240313')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a__20240313'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a__1_2_3_4_5_6_7'), '啟動盤點')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a__1_2_3_4_5_6_7'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a_Task 22'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/img_catc_Any_4'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a_'), '流通')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a__1'), '盤點')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a__1_2'), '盤點維護')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a__1_2'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a_3'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a__20240313_1'), '測試盤點_20240313')

WebUI.verifyElementText(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/td__1'), '啟動盤點')

WebUI.takeFullPageScreenshot()

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/建立盤點啟動盤點/Page_(15trunk)/a__1_2_3_4_5_6_7_8'))

WebUI.closeBrowser()

