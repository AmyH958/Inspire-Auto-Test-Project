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

WebUI.openBrowser('http://10.11.20.15/inspireapp/logon')

WebUI.enableSmartWait()

WebUI.maximizeWindow(FailureHandling.STOP_ON_FAILURE)

WebUI.setText(findTestObject('Object Repository/統計/採購統計/書商採購統計/UI顯示正確/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/統計/採購統計/書商採購統計/UI顯示正確/Page_(15trunk)/input_(15trunk)_member_pwd'), 
    'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/統計/採購統計/書商採購統計/UI顯示正確/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/統計/採購統計/書商採購統計/UI顯示正確/Page_(15trunk)/a_'), '統計')

WebUI.enhancedClick(findTestObject('Object Repository/統計/採購統計/書商採購統計/UI顯示正確/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/統計/採購統計/書商採購統計/UI顯示正確/Page_(15trunk)/a__1'), '採購統計')

WebUI.enhancedClick(findTestObject('Object Repository/統計/採購統計/書商採購統計/UI顯示正確/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/統計/採購統計/書商採購統計/UI顯示正確/Page_(15trunk)/a__1_2'), '書商採購統計')

WebUI.enhancedClick(findTestObject('Object Repository/統計/採購統計/書商採購統計/UI顯示正確/Page_(15trunk)/a__1_2'))

WebUI.enhancedClick(findTestObject('Object Repository/統計/採購統計/書商採購統計/UI顯示正確/Page_(15trunk)/div_function createPopEditforcheck(formIds,_1733b0'))

WebUI.setText(findTestObject('Object Repository/統計/採購統計/書商採購統計/UI顯示正確/Page_(15trunk)/input_-_DatePicker'), '2006/03/06')

WebUI.enhancedClick(findTestObject('Object Repository/統計/採購統計/書商採購統計/UI顯示正確/Page_(15trunk)/input_-_buttonSubmit'))

WebUI.enhancedClick(findTestObject('Object Repository/統計/採購統計/書商採購統計/UI顯示正確/Page_(15trunk)/a__1_2_3'))

WebUI.closeBrowser()

