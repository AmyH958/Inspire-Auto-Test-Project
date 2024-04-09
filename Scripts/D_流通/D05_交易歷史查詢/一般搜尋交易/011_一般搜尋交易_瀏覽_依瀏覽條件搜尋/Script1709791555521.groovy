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

WebUI.setText(findTestObject('Object Repository/流通/交易歷史查詢/一般/依瀏覽條件搜尋/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/流通/交易歷史查詢/一般/依瀏覽條件搜尋/Page_(15trunk)/input_(15trunk)_member_pwd'), 
    'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依瀏覽條件搜尋/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/一般/依瀏覽條件搜尋/Page_(15trunk)/a_'), '流通')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依瀏覽條件搜尋/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/一般/依瀏覽條件搜尋/Page_(15trunk)/a__1'), '交易歷史查詢')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依瀏覽條件搜尋/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/一般/依瀏覽條件搜尋/Page_(15trunk)/a__1_2'), '一般搜尋交易')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依瀏覽條件搜尋/Page_(15trunk)/a__1_2'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依瀏覽條件搜尋/Page_(15trunk)/input__formSubmitSearch'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/一般/依瀏覽條件搜尋/Page_(15trunk)/b_'), '瀏覽')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依瀏覽條件搜尋/Page_(15trunk)/b_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/一般/依瀏覽條件搜尋/Page_(15trunk)/b__1'), '瀏覽條件:')

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/交易歷史查詢/一般/依瀏覽條件搜尋/Page_(15trunk)/select_'), '2', true)

WebUI.setText(findTestObject('Object Repository/流通/交易歷史查詢/一般/依瀏覽條件搜尋/Page_(15trunk)/input__listField'), '楊')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依瀏覽條件搜尋/Page_(15trunk)/input__browse'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依瀏覽條件搜尋/Page_(15trunk)/td_'))

WebUI.takeFullPageScreenshot()

WebUI.verifyTextPresent('楊', false)

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依瀏覽條件搜尋/Page_(15trunk)/a_2'))

WebUI.setText(findTestObject('Object Repository/流通/交易歷史查詢/一般/依瀏覽條件搜尋/Page_(15trunk)/input__cp'), '3')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依瀏覽條件搜尋/Page_(15trunk)/input__cp'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依瀏覽條件搜尋/Page_(15trunk)/a__1_2_3'))

WebUI.closeBrowser()

