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

WebUI.setText(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_(15trunk)/input_(15trunk)_member_pwd'), 
    'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_(15trunk)/a_'), '場地設備')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_(15trunk)/a__1'), '維護')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_(15trunk)/a__1_2'), '單項維護')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_(15trunk)/a__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_(15trunk)/b_'), '查詢')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_(15trunk)/b_'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_(15trunk)/input__formSubmitSearch'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_(15trunk)/img__Any_2'))

WebUI.switchToWindowUrl('http://10.11.20.15/inspireapp/MaintainSingle,crsMaintainSingle.$DirectLink.sdirect?sp=l921')

WebUI.selectOptionByValue(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_Details/select_'), '1', true)

WebUI.setText(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_Details/textarea_20000'), '  遺失賠20000元  \n賠償2萬')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_Details/td_'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_Details/a_'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_Details/span_'))

WebUI.selectOptionByValue(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_Details/select_'), '0', true)

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_Details/a_'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_Details/span_'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_Details/img__Any_7'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_Details/a__1'))

WebUI.switchToDefaultContent(FailureHandling.STOP_ON_FAILURE)

WebUI.setText(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_(15trunk)/input__field1'), '講桌')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_(15trunk)/input__formSubmitSearch'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_(15trunk)/img__Any_2'))

WebUI.switchToWindowTitle('Details')

WebUI.setText(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_Details/input__TextField_0'), 'CC20240320')

WebUI.setText(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_Details/textarea_202403190454'), '講桌202403200454')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_Details/a_'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_Details/span_'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_Details/img__Any_7'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_Details/a__1'))

WebUI.switchToDefaultContent(FailureHandling.STOP_ON_FAILURE)

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_(15trunk)/span_'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/編輯UI介面顯示正確/Page_(15trunk)/a__1_2_3'))

WebUI.closeBrowser()

