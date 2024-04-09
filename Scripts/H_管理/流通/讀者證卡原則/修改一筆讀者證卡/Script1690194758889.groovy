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

WebUI.setText(findTestObject('Object Repository/管理/流通/讀者證卡原則/修改一筆讀者證卡資料/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/管理/流通/讀者證卡原則/修改一筆讀者證卡資料/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/修改一筆讀者證卡資料/input_(15trunk)_Submit'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/修改一筆讀者證卡資料/a_'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/修改一筆讀者證卡資料/a__1'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/修改一筆讀者證卡資料/a__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/修改一筆讀者證卡資料/td_1234567890'), '1234567890')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/修改一筆讀者證卡資料/a_PrefixSufixNumberLocationPatron classNati_e8c860'))

WebUI.scrollToElement(findTestObject('Page_(15trunk)/td_'), 2)

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/修改一筆讀者證卡資料/td_'), '選擇控制項:')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/修改一筆讀者證卡資料/img__Any_4'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/修改一筆讀者證卡資料/img__Any_4'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/修改一筆讀者證卡資料/img_Sufix_Any_6'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/修改一筆讀者證卡資料/img__Any_4'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/修改一筆讀者證卡資料/img__Any_4'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/修改一筆讀者證卡資料/a__1_2_3'), '修改/存檔')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/修改一筆讀者證卡資料/a__1_2_3'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/修改一筆讀者證卡資料/div_'), '更動已儲存')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/修改一筆讀者證卡資料/span_'), '確定')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/修改一筆讀者證卡資料/span_'))

WebUI.takeFullPageScreenshotAsCheckpoint('確認修改讀者證卡成功')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/修改一筆讀者證卡資料/td__1'), '規則:')

WebUI.scrollToElement(findTestObject('Page_(15trunk)/td__1'), 1)

WebUI.takeFullPageScreenshotAsCheckpoint('查看修改讀者證卡')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/修改一筆讀者證卡資料/a__1_2_3_4'))

WebUI.closeBrowser()

