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

WebUI.setText(findTestObject('Object Repository/清單/編目清單/新版書標維護/列印清單維護/正常新增/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/清單/編目清單/新版書標維護/列印清單維護/正常新增/Page_(15trunk)/input_(15trunk)_member_pwd'), 
    'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/清單/編目清單/新版書標維護/列印清單維護/正常新增/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/清單/編目清單/新版書標維護/列印清單維護/正常新增/Page_(15trunk)/a_'), '清單')

WebUI.enhancedClick(findTestObject('Object Repository/清單/編目清單/新版書標維護/列印清單維護/正常新增/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/清單/編目清單/新版書標維護/列印清單維護/正常新增/Page_(15trunk)/a__1'), '編目清單')

WebUI.enhancedClick(findTestObject('Object Repository/清單/編目清單/新版書標維護/列印清單維護/正常新增/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/清單/編目清單/新版書標維護/列印清單維護/正常新增/Page_(15trunk)/a__1_2'), '新版書標維護')

WebUI.enhancedClick(findTestObject('Object Repository/清單/編目清單/新版書標維護/列印清單維護/正常新增/Page_(15trunk)/a__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/清單/編目清單/新版書標維護/列印清單維護/正常新增/Page_(15trunk)/b_'), '待印清單維護')

WebUI.enhancedClick(findTestObject('Object Repository/清單/編目清單/新版書標維護/列印清單維護/正常新增/Page_(15trunk)/img__ImageSearch'))

WebUI.enhancedClick(findTestObject('Object Repository/清單/編目清單/新版書標維護/列印清單維護/正常新增/Page_(15trunk)/a__1_2_3'))

WebUI.switchToWindowTitle('Details')

WebUI.setText(findTestObject('Object Repository/清單/編目清單/新版書標維護/列印清單維護/正常新增/Page_Details/input__TextField'), '測試清單20240412')

WebUI.enhancedClick(findTestObject('Object Repository/清單/編目清單/新版書標維護/列印清單維護/正常新增/Page_Details/input__TextField'))

WebUI.enhancedClick(findTestObject('Object Repository/清單/編目清單/新版書標維護/列印清單維護/正常新增/Page_Details/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/清單/編目清單/新版書標維護/列印清單維護/正常新增/Page_Details/div_'), '儲存成功')

WebUI.enhancedClick(findTestObject('Object Repository/清單/編目清單/新版書標維護/列印清單維護/正常新增/Page_Details/span_'))

WebUI.switchToWindowUrl('http://10.11.20.15/inspireapp/BookLabelsOptionReport.page?init=1')

WebUI.enhancedClick(findTestObject('Object Repository/清單/編目清單/新版書標維護/列印清單維護/正常新增/Page_(15trunk)/input__list_buttonSubmit'), 
    FailureHandling.CONTINUE_ON_FAILURE)

WebUI.enhancedClick(findTestObject('Object Repository/清單/編目清單/新版書標維護/列印清單維護/正常新增/Page_(15trunk)/td_20240412'), FailureHandling.CONTINUE_ON_FAILURE)

WebUI.enhancedClick(findTestObject('Object Repository/清單/編目清單/新版書標維護/列印清單維護/正常新增/Page_(15trunk)/a__1_2_3_4'))

WebUI.closeBrowser()

